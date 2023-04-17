use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// ```rust
/// let message = Message {
///     sender: "Alice".to_string(),
///     body: b"Hello, world!".to_vec(),
///     timestamp: SystemTime::now()
///         .duration_since(SystemTime::UNIX_EPOCH)
///         .unwrap()
///         .as_secs(),
/// };
/// let j = serde_json::to_string(&message)?;
/// ```
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub sender: String,
    pub body: Vec<u8>,
    pub timestamp: u64,
}

impl Message {
    pub fn new(sender: String, body: Vec<u8>) -> Self {
        Self {
            sender,
            body,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.sender,
            String::from_utf8_lossy(&self.body)
        )
    }
}
