use std::net::SocketAddr;
use dotenvy::dotenv;
use handler::redis_handler;
use tokio::{net::TcpListener, task};

mod handler;
pub mod authorization;
pub mod session;

#[tokio::main]
async fn main() {
    dotenv().expect(".env nto found");
    // サーバーポート 
    const PORT: i32 = 8891;
    // サーバーアドレス
    let addr = format!("127.0.0.1:{}", PORT).parse::<SocketAddr>().unwrap();

    // セッションリスト 
    let sessions = session::session::create_sessions();

    // redis接続
    let redis = handler::redis_handler::new_client("redis://127.0.0.1").await;
    task::spawn(redis_handler::handle(redis, sessions.clone()));

    // lisnterにバインド
    let listener = TcpListener::bind(addr).await.expect("binding error.");
    println!("WebSocket server lisntening on :{}", addr);

    // サーバー実行
    while let Ok((stream, client_addr)) = listener.accept().await {
        tokio::spawn(handler::websocket_handler::handle(stream, client_addr, sessions.clone()));
    }
}
