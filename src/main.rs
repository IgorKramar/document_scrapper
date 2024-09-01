use std::str;
use std::env;
use reqwest::{get, Error};
use tokio::process::Command;
use std::path::PathBuf;

// Проверяет, является ли страница SPA
fn is_spa(html: &str) -> bool {
    html.contains(r#"<script"#) && (html.contains("react") || html.contains("vue") || html.contains("angular"))
}

// Формирует путь к скрипту Puppeteer
fn get_puppeteer_script_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let project_root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let full_path = std::path::Path::new(&project_root).join("node_scripts/browser.js");

    if !full_path.exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {:?}", full_path),
        )));
    }

    Ok(full_path)
}

// Выполняет скрипт Puppeteer для получения рендеренного HTML
async fn execute_puppeteer_script(full_path: PathBuf, url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("node")
        .arg(full_path)
        .arg(url)
        .output()
        .await?;

    if output.status.success() {
        let html = str::from_utf8(&output.stdout)?;
        Ok(html.to_string())
    } else {
        let error = str::from_utf8(&output.stderr)?;
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get HTML: {}", error),
        )))
    }
}

// Обертка над двумя предыдущими функциями для получения рендеренного HTML
async fn get_html_from_puppeteer(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let script_path = get_puppeteer_script_path()?;
    execute_puppeteer_script(script_path, url).await
}

// Получает HTML через обычный HTTP-запрос
async fn fetch_html(url: &str) -> Result<String, Error> {
    let response = get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

// Основная функция программы
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://documentation.greyscript.org";

    let body_text = fetch_html(url).await?;

    if is_spa(&body_text) {
        println!("Определено как SPA, получение рендеренного HTML...");
        
        let rendered_html = get_html_from_puppeteer(url).await?;
        println!("Рендеренный HTML: {}", rendered_html);
    } else {
        println!("{}", body_text);
    }

    Ok(())
}
