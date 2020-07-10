pub async fn get(url: &str) -> anyhow::Result<String> {
    reqwest::get(url)
        .await?
        .text()
        .await
        .map_err(|e| anyhow!("error get url {}", e))
}
