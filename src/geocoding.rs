use serde::{Deserialize, Serialize};

use crate::args::WeatherArgs;

#[derive(Debug, Deserialize, Serialize)]
pub struct GeoCodingData {
    place_id: u64,
    licence: String,
    powered_by: String,
    osm_type: String,
    osm_id: u64,
    boundingbox: Vec<String>,
    lat: String,
    lon: String,
    display_name: String,
    class: String,
    r#type: String,
    importance: f64,
}

pub async fn get_location_data(args: &WeatherArgs) -> Result<Option<Vec<GeoCodingData>>, reqwest::Error> {
    let query_params = [
        ("q", args.location.clone()),
        // ("limit", String::from("1")),
    ];

    let geocoding_ans: Vec<GeoCodingData> = reqwest::Client::new()
        .get(String::from("https://geocode.maps.co/search?"))
        .query(&query_params)
        .send()
        .await?
        .json()
        .await?;

    // println!("{}", serde_json::to_string(&geocoding_ans).unwrap());

    match geocoding_ans.len() {
        0 => Ok(None),
        _ => Ok(Some(geocoding_ans)),
    }
}

pub fn get_cooordinates(data: &GeoCodingData) -> (f64, f64) {
    (data.lat.parse::<f64>().unwrap(), data.lon.parse::<f64>().unwrap(), )
}

pub fn get_display_name(data: &GeoCodingData) -> String {
    data.display_name.clone()
}



