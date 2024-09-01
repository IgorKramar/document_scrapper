use crate::domain::models::parsed_content::ParsedContent;
use kuchiki::parse_html;
use kuchiki::traits::*;

pub struct ParseHtmlService;

impl ParseHtmlService {
    pub fn parse(html: &str) -> ParsedContent {
        let document = parse_html().one(html);
        let mut contents = Vec::new();

        for css_match in document.select("p, h1, h2, h3").unwrap() {
            contents.push(css_match.text_contents());
        }

        ParsedContent::new(contents)
    }
}
