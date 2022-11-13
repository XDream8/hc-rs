use reqwest;

#[tokio::main]
pub async fn fetch(uri: &str, body: &mut String) -> Result<(), reqwest::Error> {
    let resp = reqwest::get(uri).await?;
    *body = resp.text().await?;
    Ok(())
}
