use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpResponse,
};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use futures_util::future::LocalBoxFuture;
use log::{error, warn};
use serde_json::json;
use std::future::{ready, Ready};
use std::task::{Context, Poll};

use crate::service::database::models::Session;

type DbPool = Pool<ConnectionManager<PgConnection>>;

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
        println!("route: {:?}", req.path());
        if req.path() == "/api/sign-in" || req.path() == "/api/sign-up" {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_boxed_body())
            });
        }

        let session_header = req.headers().get("x-session-token").cloned();
        let session_token = match session_header {
            Some(token) => token.to_str().unwrap_or_default().to_string(),
            None => {
                let error_response = json!({
                    "status": "error",
                    "message": "Missing session token"
                });
                let response = HttpResponse::Unauthorized()
                    .content_type("application/json")
                    .body(error_response.to_string());

                return Box::pin(
                    async move { Ok(req.into_response(response).map_into_boxed_body()) },
                );
            }
        };
        println!("session token: {:#?}", session_token);

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

        let sess = match Session::get_by_token(&mut conn, session_token) {
            Ok(session) => session,
            Err(e) => {
                warn!("Authorized access atempted: {:#?}", e);
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
        println!("session: {:#?}", sess);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_boxed_body())
        })
    }
}
