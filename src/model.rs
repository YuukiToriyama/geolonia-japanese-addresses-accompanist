use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct ApiResponse {
    meta: HashMap<String, usize>,
    pub(crate) data: Vec<Prefecture>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Prefecture {
    code: usize,
    pub(crate) pref: String,
    point: Vec<f64>,
    pub(crate) cities: Vec<City>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct City {
    code: usize,
    pub(crate) county: Option<String>,
    pub(crate) city: Option<String>,
    pub(crate) ward: Option<String>,
    point: Vec<f64>,
}

impl City {
    pub(crate) fn get_city_name(&self) -> Option<String> {
        if let Some(county) = &self.county {
            if let Some(city) = &self.city {
                Some(format!("{}{}", county, city))
            } else {
                panic!("City name is missing for county: {}", county);
            }
        } else {
            if let Some(city) = &self.city {
                if let Some(ward) = &self.ward {
                    return Some(format!("{}{}", city, ward));
                }
                Some(city.clone())
            } else {
                panic!("City name and county name are missing");
            }
        }
    }
}
