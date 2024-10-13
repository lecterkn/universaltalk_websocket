use std::net::SocketAddr;

use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{protocol::frame::coding::CloseCode, Message};
use uuid::Uuid;

use crate::authorization;
use crate::session::session::{new_session, Sessions};

// WebSocketのハンドラ
pub async fn handle(stream: TcpStream, client_addr: SocketAddr, sessions: Sessions) {
    let ws_stream = tokio_tungstenite::accept_async(stream).await.expect("handlshake error");
    println!("connection from: {}", client_addr);
    
    let (mut outgoing, mut incoming) = ws_stream.split();
    let message = incoming.next().await;
    println!("message received!");
    if let Some(Ok(Message::Text(token))) = message {
        println!("message received: {}", token);
        match authorization::authenticator::authorize_jwt(&token) {
            Ok(claims) => {
                // 認証成功メッセージを送信
                let _ = outgoing.send(Message::Text(token)).await;
                let id = Uuid::parse_str(&claims.sub).unwrap();
                // セッションを登録
                let mut receiver = new_session(sessions, id);
                println!("authorized client: {}", id);
                // レシーバーからメッセージを受信
                while let Ok(msg) = receiver.recv().await {
                    // クライアントに送信
                    if outgoing.send(Message::Text(msg)).await.is_err() {
                        println!("[{}]: failed to send message", id);
                    }
                }
            }
            Err(_) => {
                // 接続を終了
                println!("authorization error!");
                let _ = outgoing.send(Message::Close(Some(tokio_tungstenite::tungstenite::protocol::CloseFrame {
                    code: CloseCode::Normal,
                    reason: "Authorization error".into()
                }))).await;
                return
            }
        }
    }
}