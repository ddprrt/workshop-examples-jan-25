use std::{collections::HashMap, sync::Arc};

mod traits;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, Query, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Extension, Router,
};
use futures::{SinkExt, StreamExt};
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    RwLock,
};
use tower_http::services::ServeDir;

pub fn router() -> Router {
    let (tx, _): (Sender<Message>, Receiver<Message>) = broadcast::channel(100);
    let static_dir = get_service(ServeDir::new("./static")).handle_error(handle_error);
    Router::new()
        .route("/ws", get(ws_handler))
        .route("/hello", get(hello))
        .route("/hi/:name", get(hi))
        .layer(Extension(Arc::new(RwLock::new(tx))))
        .fallback_service(static_dir)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(lock): Extension<Arc<RwLock<Sender<Message>>>>,
) -> impl IntoResponse {
    let tx = lock.read().await.clone();
    let rx = tx.subscribe();
    ws.on_upgrade(|ws| handle_socket(ws, tx, rx))
}

async fn handle_socket(ws: WebSocket, tx: Sender<Message>, mut rx: Receiver<Message>) {
    println!("Connected");
    let (mut sink, mut stream) = ws.split();
    loop {
        tokio::select! {
            msg = stream.next() => {
                if let Some(Ok(result)) = msg {
                    let result = tx.send(result);
                    if result.is_err() {
                        break;
                    }
                }
            }
            msg = rx.recv() => {
                if let Ok(msg) = msg {
                    let result = sink.send(msg).await;
                    if result.is_err() {
                        break;
                    }
                }
            }
        }
    }
    println!("Disconnected");
}

async fn hello(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let name = params
        .get("name")
        .map(ToOwned::to_owned)
        .unwrap_or("World".to_string());
    format!("Hello {name}")
}
async fn hi(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello {name}"))
}

async fn handle_error(err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err))
}
