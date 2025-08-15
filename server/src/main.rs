mod db;

use std::{env, sync::Arc};

use axum::{
    Router,
    extract::{State, WebSocketUpgrade, ws::Message},
    response::IntoResponse,
    routing::get,
};

use crate::db::Database;

pub struct AppState {
    db: Database,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    let db = Database::connect(&url).await.unwrap();

    let app_state = Arc::new(AppState { db });

    let router = Router::new()
        .route("/", get(async || "Hello, World!"))
        .route("/ws", get(ws_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn ws_handler(
    State(state): State<Arc<AppState>>,
    ws_upgrade: WebSocketUpgrade,
) -> impl IntoResponse {
    ws_upgrade.on_upgrade(|mut ws| async move {
        while let Some(msg) = ws.recv().await {
            let msg = match msg {
                Ok(msg) => msg,
                Err(err) => {
                    tracing::warn!("Websocket recv error: {}", err);
                    break;
                }
            };

            match msg {
                Message::Text(text) => {
                    ws.send(Message::Text(text)).await.unwrap();
                }
                Message::Binary(_) | Message::Ping(_) | Message::Pong(_) | Message::Close(_) => {}
            }
        }
    })
}
