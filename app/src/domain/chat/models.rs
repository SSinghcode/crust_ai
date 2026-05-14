use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub id: u64,
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSummary {
    pub unid: String,
    pub title: String,
    pub preview: String,
    pub message_count: u32,
    pub mode: String,
    pub created_at: String,
}
