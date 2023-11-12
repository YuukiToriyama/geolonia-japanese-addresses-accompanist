use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const GEOLONIA_API_MASTER: &str = "https://geolonia.github.io/japanese-addresses/api/ja.json";
    let response = reqwest::blocking::get(GEOLONIA_API_MASTER).unwrap();
    let json = response.json::<HashMap<String, Vec<String>>>().unwrap();
    println!("{:#?}", json);
    Ok(())
}
