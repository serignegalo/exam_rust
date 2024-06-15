pub struct Message {
    pub user: String,
    pub content: String,
}

impl Message {
    pub fn new(ms: String, u: String) -> Message {
        Self{content: ms, user: u}
    }
    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.contains("stupid") {
            return None;
        }
        Some(self.content.as_str())
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(msg) => (true, msg),
        None => (false, "ERROR: illegal")
    }
}
