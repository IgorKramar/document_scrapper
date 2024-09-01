pub struct ParsedContent {
    pub content: Vec<String>,
}

impl ParsedContent {
    pub fn new(content: Vec<String>) -> Self {
        Self { content }
    }
}
