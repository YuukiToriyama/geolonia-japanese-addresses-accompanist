use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = fetch_geolonia_api_master().await.unwrap();
    println!("{:#?}", json);
    Ok(())
}

async fn fetch_geolonia_api_master() -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    const GEOLONIA_API_MASTER: &str = "https://geolonia.github.io/japanese-addresses/api/ja.json";
    let response = reqwest::get(GEOLONIA_API_MASTER).await.unwrap();
    let json = response.json::<HashMap<String, Vec<String>>>().await.unwrap();
    Ok(json)
}