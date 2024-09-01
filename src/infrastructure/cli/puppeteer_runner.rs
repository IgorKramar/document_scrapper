use std::env;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::str;
use tokio::process::Command;

pub struct PuppeteerRunner;

impl PuppeteerRunner {
    fn get_embedded_puppeteer_binary() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let binary_data = include_bytes!("../../../bin/puppeteer_js");
        let tmp_path = env::temp_dir().join("puppeteer_js");
        let mut file = File::create(&tmp_path)?;
        file.write_all(binary_data)?;
        std::fs::set_permissions(&tmp_path, std::fs::Permissions::from_mode(0o755))?;
        Ok(tmp_path)
    }

    pub async fn run(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let binary_path = Self::get_embedded_puppeteer_binary()?;
        let output = Command::new(binary_path).arg(url).output().await?;

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
}
