use actix_ws::{Message, Session};
use std::{collections::HashMap, io, sync::Arc, time::Instant};
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

use crate::shared::utils::clone_websocket_message;

pub type ConnId = Uuid;
pub type SessionToken = String;

struct Connection {
    session_token: SessionToken,
    last_heartbeat: Instant,
    #[allow(dead_code)]
    sender: mpsc::UnboundedSender<Message>,
    session: Session,
}

pub struct WebSocketManager {
    connections: Arc<RwLock<HashMap<ConnId, Connection>>>,
    sessions: Arc<RwLock<HashMap<SessionToken, Vec<ConnId>>>>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        WebSocketManager {
            connections: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn connect(
        &self,
        session_token: &SessionToken,
        session: Session,
    ) -> io::Result<ConnId> {
        let conn_id = ConnId::new_v4();
        let (tx, mut rx) = mpsc::unbounded_channel();

        let connection = Connection {
            session_token: session_token.clone(),
            last_heartbeat: Instant::now(),
            sender: tx,
            session,
        };

        self.connections.write().await.insert(conn_id, connection);
        self.sessions
            .write()
            .await
            .entry(session_token.clone())
            .or_default()
            .push(conn_id);

        tokio::spawn({
            let connections = Arc::clone(&self.connections);
            async move {
                while let Some(msg) = rx.recv().await {
                    log::info!("Received message {:?} from connection: {:?}", msg, conn_id);
                    if let Err(e) =
                        Self::send_message_to_connection(&connections, conn_id, msg).await
                    {
                        log::error!("Failed to send message: {:?}", e);
                        break;
                    }
                }
                connections.write().await.remove(&conn_id);
            }
        });

        Ok(conn_id)
    }

    pub async fn disconnect(&self, conn_id: ConnId) -> io::Result<()> {
        if let Some(conn) = self.connections.write().await.remove(&conn_id) {
            let mut sessions = self.sessions.write().await;
            if let Some(conn_ids) = sessions.get_mut(&conn.session_token) {
                conn_ids.retain(|&id| id != conn_id);
                if conn_ids.is_empty() {
                    sessions.remove(&conn.session_token);
                }
            }
        }
        Ok(())
    }

    pub async fn send_message_to_session(
        &self,
        session_token: &SessionToken,
        message: Message,
        sender_conn_id: ConnId,
    ) -> io::Result<()> {
        let sessions = self.sessions.read().await;
        let connections = self.connections.read().await;

        if let Some(conn_ids) = sessions.get(session_token) {
            for &conn_id in conn_ids {
                if conn_id == sender_conn_id {
                    continue;
                }
                log::info!(
                    "sender_conn_id: {:?}, conn_id: {:?}",
                    sender_conn_id,
                    conn_id
                );
                if let Some(conn) = connections.get(&conn_id) {
                    log::info!("Sending message {:?} to connection: {:?}", message, conn_id);
                    let mut session = conn.session.clone();
                    match message {
                        Message::Text(ref text) => {
                            session.text(text.clone()).await.map_err(|e| {
                                io::Error::new(
                                    io::ErrorKind::Other,
                                    format!("Failed to send message: {:?}", e),
                                )
                            })?;
                        }
                        Message::Binary(ref binary) => {
                            session.binary(binary.clone()).await.map_err(|e| {
                                io::Error::new(
                                    io::ErrorKind::Other,
                                    format!("Failed to send message: {:?}", e),
                                )
                            })?;
                        }
                        _ => {
                            log::error!("Received unsupported message type: {:?}", message);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn update_heartbeat(&self, conn_id: ConnId) -> io::Result<()> {
        if let Some(conn) = self.connections.write().await.get_mut(&conn_id) {
            conn.last_heartbeat = Instant::now();
        }
        Ok(())
    }

    async fn send_message_to_connection(
        connections: &RwLock<HashMap<ConnId, Connection>>,
        conn_id: ConnId,
        message: Message,
    ) -> io::Result<()> {
        let mut connections = connections.write().await;
        if let Some(conn) = connections.get_mut(&conn_id) {
            match message {
                Message::Text(text) => {
                    conn.session.text(text).await.map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            format!("Failed to send message: {:?}", e),
                        )
                    })?;
                }
                Message::Binary(binary) => {
                    conn.session.binary(binary).await.map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            format!("Failed to send message: {:?}", e),
                        )
                    })?;
                }
                _ => {
                    log::error!("Received unsupported message type: {:?}", message);
                }
            }
        }
        Ok(())
    }

    pub async fn broadcast_to_session(
        &self,
        session_token: &SessionToken,
        message: Message,
    ) -> io::Result<()> {
        let sessions = self.sessions.read().await;
        let connections = self.connections.read().await;

        if let Some(conn_ids) = sessions.get(session_token) {
            for &conn_id in conn_ids {
                if let Some(conn) = connections.get(&conn_id) {
                    let cloned_message = clone_websocket_message(&message);
                    let mut session = conn.session.clone();

                    match cloned_message {
                        Message::Text(text) => {
                            session.text(text).await.map_err(|e| {
                                io::Error::new(
                                    io::ErrorKind::Other,
                                    format!("Failed to send message: {:?}", e),
                                )
                            })?;
                        }
                        Message::Binary(binary) => {
                            session.binary(binary).await.map_err(|e| {
                                io::Error::new(
                                    io::ErrorKind::Other,
                                    format!("Failed to send message: {:?}", e),
                                )
                            })?;
                        }
                        _ => {
                            log::error!("Received unsupported message type: {:?}", cloned_message);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct WebSocketManagerHandle {
    manager: Arc<WebSocketManager>,
}

impl WebSocketManagerHandle {
    pub fn new() -> Self {
        WebSocketManagerHandle {
            manager: Arc::new(WebSocketManager::new()),
        }
    }

    pub async fn connect(
        &self,
        session_token: &SessionToken,
        session: &Session,
    ) -> io::Result<ConnId> {
        self.manager.connect(session_token, session.clone()).await
    }

    pub async fn disconnect(&self, conn_id: ConnId) -> io::Result<()> {
        self.manager.disconnect(conn_id).await
    }

    pub async fn send_message_to_session(
        &self,
        session_token: &SessionToken,
        message: Message,
        sender_conn_id: ConnId,
    ) -> io::Result<()> {
        self.manager
            .send_message_to_session(session_token, message, sender_conn_id)
            .await
    }

    pub async fn update_heartbeat(&self, conn_id: ConnId) -> io::Result<()> {
        self.manager.update_heartbeat(conn_id).await
    }

    pub async fn broadcast_to_session(
        &self,
        session_token: &SessionToken,
        message: Message,
    ) -> io::Result<()> {
        self.manager
            .broadcast_to_session(session_token, message)
            .await
    }
}
