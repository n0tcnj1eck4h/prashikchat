use axum::{
    Router,
    extract::{
        WebSocketUpgrade,
        ws::Message::{self, Text},
    },
    response::IntoResponse,
    routing::get,
};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(async || "Hello, World!"))
        .route("/ws", get(ws_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn ws_handler(ws_upgrade: WebSocketUpgrade) -> impl IntoResponse {
    ws_upgrade.on_upgrade(|mut ws| async move {
        while let Some(msg) = ws.recv().await {
            let msg = msg.unwrap();
            match msg {
                Message::Text(text) => {
                    ws.send(Message::Text(text)).await.unwrap();
                }
                Message::Binary(_) | Message::Ping(_) | Message::Pong(_) | Message::Close(_) => {}
            }
        }
    })
}
