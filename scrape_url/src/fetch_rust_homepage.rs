use std::fs;

pub fn fetch_rust_homepage_then_convert_to_markdown() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = request(url);

    println!("Convert htmo to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}", output);

    Ok(())
}

fn request(url: &str) -> String {
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    body
}
