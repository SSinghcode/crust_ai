pub struct ChatRoutes;

impl ChatRoutes {
    pub fn base_url() -> &'static str {
        "/chats"
    }

    pub fn detail_url(id: &str) -> String {
        format!("/chats/{id}")
    }

    pub fn label() -> &'static str {
        "Chat"
    }
}
