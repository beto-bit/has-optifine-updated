use std::path::Path;

use select::document::Document;
use select::predicate::{Class, Name};

const URL: &str = "https://www.optifine.net/downloads";

#[tokio::main]
async fn main() {
    let response = fetch_url(URL).await;
    let version = extract_version(&response).expect("Couldn't parse HTML");

    println!("Current OptiFine version: {version}");
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

fn load_cached_data(path: impl AsRef<Path>) -> Option<String> {
    todo!()
}

fn store_data(data: &str, path: impl AsRef<Path>) -> std::io::Result<()> {
    todo!()
}
