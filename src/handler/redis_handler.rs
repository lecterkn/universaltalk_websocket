use futures::StreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::session::session;

const CHANNEL_BROADCAST: &str = "broadcast";

#[derive(Serialize, Deserialize, Debug)]
struct RedisMessage {
    src: Uuid,
    event: String,
    message: String,
}

pub async fn new_client(url: &str) -> redis::Client {
    return redis::Client::open(url).expect("redis connection error");
}

pub async fn handle(client: redis::Client, sessions: session::Sessions) {
    let mut sub = client.get_async_pubsub().await.expect("failed to get async pubsub");
    sub.subscribe(CHANNEL_BROADCAST).await.expect("failed to subscribe broadcast channel");
    loop {
        let msg = sub.on_message().next().await;
        if let Some(message) = msg {
            let payload: String = message.get_payload().expect("invalid message payload");
            println!("payload: {}", payload);
            if let Ok(redis_message) = serde_json::from_str::<RedisMessage>(&payload) {
                println!("success to format");
                let session_map = sessions.lock().unwrap();
                println!("session size: {}", session_map.keys().len());
                for session_id in session_map.keys() {
                    println!("session id: {}", *session_id);
                    if redis_message.src != *session_id {
                        let _ = session_map.get(session_id).unwrap().send(payload.clone());
                    }
                    else {
                        println!("unmatched user: {}", *session_id);
                    }
                }
            }
        }
    }
}