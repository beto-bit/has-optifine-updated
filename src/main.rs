use select::{document::Document, predicate::{Class, Name}};

const URL: &str = "https://www.optifine.net/downloads";

fn main() {
    let response = fetch_url(URL);
    let version = extract_version(&response).unwrap();

    println!("Current OptiFine version: {}", version);
}

fn fetch_url(url: &str) -> String {
    reqwest::blocking::get(url)
        .expect("Couldn't fetch data")
        .text()
        .expect("Error decoding data")
}

fn extract_version(html: &str) -> Option<String> {
    let document = Document::from(html);
    let html_span = document.find(Class("downloads")).nth(0)?;
    let html_h2 = html_span.find(Name("h2")).nth(0)?;

    Some(html_h2.text())
}
