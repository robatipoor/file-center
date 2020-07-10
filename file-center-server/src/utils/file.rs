use tokio::fs;

pub async fn read_file(path: &str) -> anyhow::Result<String> {
    fs::read_to_string(path)
        .await
        .map_err(|e| anyhow!("read file error {}", e))
}

pub async fn write_file(path: &str, contents: &[u8]) -> anyhow::Result<()> {
    fs::write(path, contents)
        .await
        .map_err(|e| anyhow!("write file error {}", e))
}
