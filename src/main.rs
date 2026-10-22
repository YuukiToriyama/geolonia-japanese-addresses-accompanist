mod model;

use crate::model::ApiResponse;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = Path::new("./public");
    fs::create_dir_all(base_dir)?;

    for prefecture in fetch_geolonia_api_master().await? {
        let prefecture_dir = base_dir.join(&prefecture.name);
        fs::create_dir_all(&prefecture_dir)?;

        let file_path = prefecture_dir.join("master.json");
        let file = fs::File::create(&file_path)?;
        serde_json::to_writer(&file, &prefecture)?;

        println!("Generated file: {}", file_path.display());
    }
    Ok(())
}

async fn fetch_geolonia_api_master() -> Result<Vec<Prefecture>, Box<dyn std::error::Error>> {
    const GEOLONIA_API_MASTER: &str = "https://japanese-addresses-v2.geoloniamaps.com/api/ja.json";
    let response = reqwest::get(GEOLONIA_API_MASTER)
        .await?
        .error_for_status()?;
    let json = response.json::<ApiResponse>().await?;
    let prefectures = json
        .data
        .into_iter()
        .map(|prefecture| Prefecture {
            name: prefecture.pref,
            cities: prefecture
                .cities
                .into_iter()
                .filter_map(|city| city.get_city_name())
                .collect(),
        })
        .collect();
    Ok(prefectures)
}

#[derive(Serialize, Debug)]
struct Prefecture {
    name: String,
    cities: Vec<String>,
}
