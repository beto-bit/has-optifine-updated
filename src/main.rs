use select::document::Document;
use select::predicate::{Class, Name};

const URL: &str = "https://www.optifine.net/downloads";

fn main() {
    let response = fetch_url(URL);
    let version = extract_version(&response).expect("Couldn't parse HTML");

    println!("Current OptiFine version: {version}");
}

fn fetch_url(url: &str) -> String {
    reqwest::blocking::get(url)
        .expect("Couldn't fetch data")
        .text()
        .expect("Error decoding data")
}

fn extract_version(html: &str) -> Option<String> {
    let document = Document::from(html);
    let html_span = document.find(Class("downloads")).next()?;
    let html_h2 = html_span.find(Name("h2")).next()?;

    Some(html_h2.text())
}
