use crate::HTTP_CLIENT;

// colored output
use colored::*;

#[tokio::main]
pub async fn fetch(uri: &str, body: &mut String) -> Result<(), reqwest::Error> {
    let resp = HTTP_CLIENT.get(uri).send().await?;
    if resp.status() != 200 {
        eprintln!("{} ({}) {}: {}",
            "fetching".red().bold(),
            uri.yellow(),
            "failed".red().bold(),
            format!("{}", resp.status()).red().bold(),
        );
    }
    else {
        *body = resp.text().await?;
    }
    Ok(())
}
