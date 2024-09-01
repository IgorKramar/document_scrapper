use crate::domain::models::html_content::HtmlContent;
use crate::infrastructure::cli::puppeteer_runner::PuppeteerRunner;

pub struct PuppeteerHtmlService;

impl PuppeteerHtmlService {
    pub async fn get_rendered_html(url: &str) -> Result<HtmlContent, Box<dyn std::error::Error>> {
        let content = PuppeteerRunner::run(url).await?;
        Ok(HtmlContent::new(content))
    }
}
