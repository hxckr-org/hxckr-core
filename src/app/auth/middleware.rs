use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use log::{error, warn};
use serde_json::json;
use std::task::{Context, Poll};
use std::{
    collections::HashMap,
    future::{ready, Ready},
};
use uuid::Uuid;

use crate::service::database::{conn::DbPool, models::Session};

#[allow(dead_code)]
pub struct SessionInfo {
    pub token: String,
    pub user_id: Uuid,
}
pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static + MessageBody,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static + MessageBody,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if req.path() == "/api/sign-in"
            || req.path() == "/api/sign-up"
            || req.path() == "/api/health"
        {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_boxed_body())
            });
        }

        // We're using the same middleware for both HTTP and WebSocket requests
        // but websocket connection from web-browsers have a limitation on the request headers.
        // The WebSocket API does not support custom headers during the handshake.
        // This is a browser security limitation.
        // So, we need to get the session token from the query params for websocket requests
        let session_token = if req.headers().get("Upgrade").is_some() {
            // WebSocket request: get token from query params
            let query_map = req
                .query_string()
                .split('&')
                .map(|param| {
                    let parts = param.splitn(2, '=').collect::<Vec<&str>>();
                    (parts[0], parts.get(1).unwrap_or(&"").to_string())
                })
                .collect::<HashMap<&str, String>>();

            query_map.get("token").cloned().unwrap_or_default()
        } else {
            // HTTP/HTTPS request: get token from headers
            req.headers()
                .get("x-session-token")
                .and_then(|token| token.to_str().ok())
                .map(String::from)
                .unwrap_or_default()
        };

        if session_token.is_empty() {
            let error_response = json!({
                "status": "error",
                "message": "Missing session token"
            });
            let response = HttpResponse::Unauthorized()
                .content_type("application/json")
                .body(error_response.to_string());
            return Box::pin(async move { Ok(req.into_response(response).map_into_boxed_body()) });
        }

        let pool = req
            .app_data::<web::Data<DbPool>>()
            .expect("DB Pool not found in request");

        let conn_result = pool.get();
        let mut conn = match conn_result {
            Ok(conn) => conn,
            Err(e) => {
                let err = actix_web::error::ErrorInternalServerError("Internal Server Error");
                error!("Failed to connect to DB pool: {:#?}", e);
                return Box::pin(async move { Err(err) });
            }
        };

        match Session::get_by_token(&mut conn, session_token) {
            Ok(session) => {
                let time_now = chrono::Utc::now().naive_utc();
                if session.expires_at < time_now {
                    let error_response = json!({
                        "status": "error",
                        "message": "Unauthorized. Session token expired!"
                    });
                    let response = HttpResponse::Unauthorized()
                        .content_type("application/json")
                        .body(error_response.to_string());

                    return Box::pin(async move {
                        Ok(req.into_response(response).map_into_boxed_body())
                    });
                }
                let session_info = SessionInfo {
                    token: session.token,
                    user_id: session.user_id,
                };
                req.extensions_mut().insert(session_info)
            }
            Err(e) => {
                warn!("Unauthorized access attempted: {:#?}", e);
                let error_response = json!({
                    "status": "error",
                    "message": "Unauthorized"
                });
                let response = HttpResponse::Unauthorized()
                    .content_type("application/json")
                    .body(error_response.to_string());

                return Box::pin(
                    async move { Ok(req.into_response(response).map_into_boxed_body()) },
                );
            }
        };

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_boxed_body())
        })
    }
}
