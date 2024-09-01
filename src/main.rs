mod application;
mod domain;
mod infrastructure;

use application::services::{
    FetchHtmlService, ParseHtmlService, PdfCreationService, PuppeteerHtmlService, SpaCheckService,
};

use infrastructure::pdf::lopdf_document::LopdfDocument;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://react.dev/learn";

    let html_content = FetchHtmlService::fetch_html(url).await?;

    let parsed_content = if SpaCheckService::check(&html_content) {
        println!("Определено как SPA, получение рендеренного HTML...");

        let rendered_html = PuppeteerHtmlService::get_rendered_html(url).await?;
        ParseHtmlService::parse(&rendered_html.content)
    } else {
        ParseHtmlService::parse(&html_content.content)
    };

    let document = LopdfDocument::new();
    let mut pdf_service = PdfCreationService::new(document);

    match pdf_service.create_pdf(parsed_content, "./target/documentation.pdf") {
        Ok(_) => println!("PDF успешно сохранен."),
        Err(e) => eprintln!("Ошибка при сохранении PDF: {:?}", e),
    }

    Ok(())
}
