use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ApiResponse {
    pub content: String,
}

impl ApiResponse {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl Display for ApiResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}