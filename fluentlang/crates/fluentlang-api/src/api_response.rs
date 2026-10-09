pub struct ApiResponse {
    pub content: String,
}

impl ApiResponse {
    pub fn new(content: String) -> Self {
        Self{
            content
        }
    }
}