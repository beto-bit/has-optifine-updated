use std::path::Path;

use select::document::Document;
use select::predicate::{Class, Name};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const URL: &str = "https://www.optifine.net/downloads";

#[tokio::main]
async fn main() {
    // Fetch web and data at the same time
    let (response, cached_data) = tokio::join!(fetch_url(URL), load_cached_data("Cargo.toml"));

    // Extract the version string "Minecraft 1.xx.xx"
    let version = extract_version(&response).expect("Couldn't parse HTML");

    println!("Current OptiFine version: {version}");
    println!("Data: {}", cached_data.unwrap());
}

async fn fetch_url(url: &str) -> String {
    reqwest::get(url)
        .await
        .expect("Couldn't fetch data")
        .text()
        .await
        .expect("Error decoding data")
}

fn extract_version(html: &str) -> Option<String> {
    let document = Document::from(html);
    let html_span = document.find(Class("downloads")).next()?;
    let html_h2 = html_span.find(Name("h2")).next()?;

    Some(html_h2.text())
}

async fn load_cached_data(path: impl AsRef<Path>) -> Option<String> {
    let mut file = File::open(path).await.ok()?;

    // Read the entire file to a vector, then to a String
    let mut buff = vec![];
    file.read_to_end(&mut buff).await.ok()?;
    let contents = String::from_utf8(buff).expect("Invalid UTF-8");

    Some(contents)
}

async fn store_data(data: &str, path: impl AsRef<Path>) -> std::io::Result<()> {
    let mut file = File::create(path).await?;
    file.write_all(data.as_bytes()).await?;

    Ok(())
}
