use std::path::Path;
use std::{collections::HashMap, fs};
use serde::Serialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = Path::new("./public");
    if fs::read_dir(base_dir).is_err() {
        fs::create_dir(base_dir)?;
    }
    let prefecture_list = fetch_geolonia_api_master().await?;
    for prefecture in prefecture_list {
        let prefecture_dir = base_dir.join(&prefecture.name);
        if fs::read_dir(&prefecture_dir).is_err() {
            fs::create_dir(&prefecture_dir)?;
        }
        let file = fs::File::create(prefecture_dir.join("master.json"))?;
        serde_json::to_writer(&file, &prefecture)?;
    }
    Ok(())
}

async fn fetch_geolonia_api_master() -> Result<Vec<Prefecture>, Box<dyn std::error::Error>> {
    const GEOLONIA_API_MASTER: &str = "https://geolonia.github.io/japanese-addresses/api/ja.json";
    let response = reqwest::get(GEOLONIA_API_MASTER)
        .await?
        .error_for_status()?;
    let json = response.json::<HashMap<String, Vec<String>>>().await?;
    let prefectures = json
        .into_iter()
        .map(|(name, cities)| Prefecture { name, cities })
        .collect();
    Ok(prefectures)
}

#[derive(Serialize, Debug)]
struct Prefecture {
    name: String,
    cities: Vec<String>,
}
