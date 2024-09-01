pub struct HtmlContent {
  pub content: String,
}

impl HtmlContent {
  pub fn new(content: String) -> Self {
      Self { content }
  }

  pub fn is_spa(&self) -> bool {
      self.content.contains(r#"<script"#) && 
      (self.content.contains("react") || self.content.contains("vue") || self.content.contains("angular"))
  }
}