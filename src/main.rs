use std::io::Read;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    test_request().await;
}

async fn test_request() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("http://localhost:8080/ip")
        .await?
        .text()
        .await?;

    println!("body: {:?}", body);
    let body = reqwest::get("http://ipinfo.io/ip")
        .await?
        .text()
        .await?;
    println!("ipaddress: {:?}", body);

    Ok(())
}