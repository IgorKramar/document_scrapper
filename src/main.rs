mod application;
mod domain;
mod infrastructure;

use application::services::{FetchHtmlService, SpaCheckService, PuppeteerHtmlService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://documentation.greyscript.org";

    let html_content = FetchHtmlService::fetch_html(url).await?;

    if SpaCheckService::check(&html_content) {
        println!("Определено как SPA, получение рендеренного HTML...");
        
        let rendered_html = PuppeteerHtmlService::get_rendered_html(url).await?;
        println!("Рендеренный HTML: {}", rendered_html.content);
    } else {
        println!("{}", html_content.content);
    }

    Ok(())
}