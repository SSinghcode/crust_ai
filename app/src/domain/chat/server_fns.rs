use leptos::prelude::*;

use super::models::ChatSummary;

#[server]
pub async fn fetch_chat_list() -> Result<Vec<ChatSummary>, ServerFnError> {
    let json_str = include_str!("../../../../public/chats.json");
    serde_json::from_str::<Vec<ChatSummary>>(json_str)
        .map_err(|_| ServerFnError::new("Failed to parse chat list"))
}

/// Placeholder reply — will be replaced with real AI API call.
#[server]
pub async fn get_reply(message: String) -> Result<String, ServerFnError> {
    let replies = [
        "That's a great question! Let me think through this with you.",
        "Interesting. Here's what I'd consider in this situation.",
        "Based on what you've shared, here are a few things to keep in mind.",
        "Good point. In Rust this often comes down to ownership and lifetimes.",
        "Let me break that down step by step.",
    ];
    let idx = message.len() % replies.len();
    Ok(replies[idx].to_string())
}
