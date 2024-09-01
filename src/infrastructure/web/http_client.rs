use reqwest;

pub struct HttpClient;

impl HttpClient {
    pub async fn get(url: &str) -> Result<String, reqwest::Error> {
        let response = reqwest::get(url).await?;
        Ok(response.text().await?)
    }
}