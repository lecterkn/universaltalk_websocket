use std::{collections::HashMap, sync::{Arc, Mutex}};

use tokio::sync::broadcast::{self, Receiver};
use uuid::Uuid;

pub type Sessions = Arc<Mutex<HashMap<Uuid, broadcast::Sender<String>>>>;

pub fn create_sessions() -> Sessions {
    return Arc::new(Mutex::new(HashMap::new()));
}

pub fn new_session(sessions: Sessions, id: Uuid) -> Receiver<String> {
    let (tx, rx) = broadcast::channel::<String>(8);
    sessions.lock().unwrap().insert(id, tx);
    println!("session size: {}", sessions.lock().unwrap().keys().len());
    return rx;
}