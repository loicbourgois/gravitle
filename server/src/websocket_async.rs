use crate::websocket::Senders;
use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use std::env;
use std::time::Duration;
use tokio::net::TcpListener as TokioTcpListener;
use tokio::net::TcpStream as TokioTcpStream;
use tungstenite::Message;

pub async fn serve_async(_senders: &Senders) {
    println!("  bobyy: ");
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8000".to_string());
    let try_socket = TokioTcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);
    println!("  bobyy: ");
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}

use futures_util::SinkExt;
use tokio_tungstenite::accept_async;

async fn accept_connection(stream: TokioTcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("New WebSocket connection: {}", addr);
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    let (mut ws_sender, _) = ws_stream.split();
    let mut interval = tokio::time::interval(Duration::from_millis(10));

    // Echo incoming WebSocket messages and send a message periodically every second.

    loop {
        interval.tick().await;
        ws_sender.send(Message::Text("tick".to_owned())).await.unwrap();
    }
}
