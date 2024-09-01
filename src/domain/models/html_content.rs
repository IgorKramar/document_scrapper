use regex::Regex;

pub struct HtmlContent {
    pub content: String,
}

impl HtmlContent {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn is_spa(&self) -> bool {
        let script_tag_re = Regex::new(r#"<script[^>]*>"#).unwrap();
        let spa_framework_re = Regex::new(r#"(react|vue|angular|svelte|ember)"#).unwrap();

        if let Some(script_tag) = script_tag_re.find(&self.content) {
            spa_framework_re.is_match(script_tag.as_str())
        } else {
            false
        }
    }
}
