use crate::domain::models::html_content::HtmlContent;

pub struct SpaCheckService;

impl SpaCheckService {
    pub fn check(html_content: &HtmlContent) -> bool {
        html_content.is_spa()
    }
}
