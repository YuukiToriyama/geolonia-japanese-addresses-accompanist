use std::{collections::HashMap, vec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = fetch_geolonia_api_master().await.unwrap();
    println!("{:#?}", json);
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

#[derive(Debug)]
struct Prefecture {
    name: String,
    cities: Vec<String>,
}
