pub mod fetch_html_service;
pub mod parse_html_service;
pub mod pdf_creation_service;
pub mod puppeteer_html_service;
pub mod spa_check_service;

pub use fetch_html_service::FetchHtmlService;
pub use parse_html_service::ParseHtmlService;
pub use pdf_creation_service::PdfCreationService;
pub use puppeteer_html_service::PuppeteerHtmlService;
pub use spa_check_service::SpaCheckService;
