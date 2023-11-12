use std::{collections::HashMap, fs, io::Write, vec};

use serde::Serialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const BASE_DIR: &str = "./public";
    if fs::read_dir(BASE_DIR).is_err() {
        fs::create_dir(BASE_DIR).unwrap();
    }
    let prefecture_list = fetch_geolonia_api_master().await.unwrap();
    for prefecture in prefecture_list {
        let prefecture_dir = format!("{}/{}", BASE_DIR, prefecture.name);
        if fs::read_dir(&prefecture_dir).is_err() {
            fs::create_dir(&prefecture_dir).unwrap();
        }
        let mut file = fs::File::create(format!("{}/master.json", prefecture_dir)).unwrap();
        file.write_all(
            serde_json::to_string_pretty(&prefecture)
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    }
    Ok(())
}

async fn fetch_geolonia_api_master() -> Result<Vec<Prefecture>, Box<dyn std::error::Error>> {
    const GEOLONIA_API_MASTER: &str = "https://geolonia.github.io/japanese-addresses/api/ja.json";
    let response = reqwest::get(GEOLONIA_API_MASTER).await.unwrap();
    let json = response
        .json::<HashMap<String, Vec<String>>>()
        .await
        .unwrap();
    let mut prefecture_list: Vec<Prefecture> = vec![];
    for (prefecture_name, city_list) in json {
        prefecture_list.push(Prefecture {
            name: prefecture_name,
            cities: city_list,
        })
    }
    Ok(prefecture_list)
}

#[derive(Serialize, Debug)]
struct Prefecture {
    name: String,
    cities: Vec<String>,
}
