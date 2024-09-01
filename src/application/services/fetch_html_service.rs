use crate::domain::models::html_content::HtmlContent;
use crate::infrastructure::web::http_client::HttpClient;

pub struct FetchHtmlService;

impl FetchHtmlService {
    pub async fn fetch_html(url: &str) -> Result<HtmlContent, reqwest::Error> {
        let content = HttpClient::get(url).await?;
        Ok(HtmlContent::new(content))
    }
}
