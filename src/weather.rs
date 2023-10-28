use crate::args::{DurationType, OutputMode};
use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{de::Error, Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Deserialize, Serialize)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: Vec<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct Units {
    pub air_pressure_at_sea_level: Option<String>,
    pub air_temperature: Option<String>,
    pub cloud_area_fraction: Option<String>,
    pub precipitation_amount: Option<String>,
    pub relative_humidity: Option<String>,
    pub wind_from_direction: Option<String>,
    pub wind_speed: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Meta {
    pub updated_at: DateTime<Utc>,
    pub units: Units,
}

#[derive(Deserialize, Serialize)]
pub struct Details {
    pub air_pressure_at_sea_level: Option<f64>,
    pub air_temperature: Option<f64>,
    pub cloud_area_fraction: Option<f64>,
    pub relative_humidity: Option<f64>,
    pub wind_from_direction: Option<f64>,
    pub wind_speed: Option<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct Summary {
    pub symbol_code: String,
}

#[derive(Deserialize, Serialize)]
pub struct Next12Hours {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Deserialize, Serialize)]
pub struct Next1Hour {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Deserialize, Serialize)]
pub struct Next6Hours {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Deserialize, Serialize)]
pub struct Instant {
    pub details: Details,
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub instant: Instant,
    pub next_12_hours: Option<Next12Hours>,
    pub next_1_hours: Option<Next1Hour>,
    pub next_6_hours: Option<Next6Hours>,
}

#[derive(Deserialize, Serialize)]
pub struct Timeseries {
    pub time: DateTime<Utc>,
    pub data: Data,
}

#[derive(Deserialize, Serialize)]
pub struct Properties {
    pub meta: Meta,
    pub timeseries: Vec<Timeseries>,
}

#[derive(Deserialize, Serialize)]
pub struct WeatherData {
    pub r#type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

pub async fn get_weather_data(coords: (f64, f64)) -> Result<Option<WeatherData>, reqwest::Error> {
    let query_params = [
        ("lat", coords.0),
        ("lon", coords.1),
        // ("timestep", String::from("1h")),
        // ("timeoffsets", String::from("1h")),
        // ("units", String::from("si")),
    ];
    // println!("{:?}", query_params);

    let weather_ans: WeatherData = reqwest::Client::new()
        .get("https://api.met.no/weatherapi/locationforecast/2.0/compact?")
        .header("User-Agent", "Weather-Cli/0.0.1")
        .query(&query_params)
        .send()
        .await?
        .json()
        .await?;

    // println!("{}", serde_json::to_string(&weather_ans).unwrap());
    // let text = weather_ans.text().await?;
    // println!("{}", text);
    Ok(Some(weather_ans))
}

#[derive(Debug)]
pub enum WeatherError {
    MissingData,
}

impl Display for WeatherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeatherError::MissingData => write!(f, "Missing data"),
        }
    }
}

impl WeatherData {
    pub fn display(
        &self,
        duration: DurationType,
        location_name: String,
        output_mode: OutputMode,
    ) -> Result<(), WeatherError> {

        let output = match output_mode {
            OutputMode::Compact => self.display_compact(duration, location_name),
            OutputMode::Detailed => self.display_detailed(duration, location_name),
            OutputMode::Complete => self.display_complete(duration, location_name),
        }?;
        
        println!("{}", output);
        Ok(())
    }

    fn display_complete(
        &self,
        duration: DurationType,
        location_name: String,
    ) -> Result<String, WeatherError> {
        Ok(String::new())
    }

    fn display_detailed(
        &self,
        duration: DurationType,
        location_name: String,
    ) -> Result<String, WeatherError> {
        Ok(String::new())
    }

    fn display_compact(
        &self,
        duration: DurationType,
        location_name: String,
    ) -> Result<String, WeatherError> {
        let mut output = String::new();
        let current_time = DateTime::<Utc>::from(Utc::now());
        output.push_str(&format!("Weather for {} ", location_name));
        match duration {
            DurationType::Now => {
                output.push_str(&format!("at {}\n", current_time.format("%H:%M")));
                // select the timeseries that is closest to the current time
                // print the summary and the temperature
                let closest_timeseries = self
                    .properties
                    .timeseries
                    .iter()
                    .min_by_key(|timeseries| (timeseries.time - current_time).num_seconds().abs())
                    .unwrap();
                output.push_str(&format!(
                    "{} {}°C\n",
                    format_weather_description(
                        closest_timeseries
                            .data
                            .next_1_hours
                            .as_ref()
                            .unwrap()
                            .summary
                            .symbol_code
                            .as_str()
                    ),
                    closest_timeseries
                        .data
                        .instant
                        .details
                        .air_temperature
                        .as_ref()
                        .unwrap()
                ));
            }

            DurationType::Today => {
                output.push_str(&format!("on {}\n", current_time.format("%A, %d %B")));
                // select every timeseries that is today
                // for each timeseries, print the time and the summary
                for timeseries in self.properties.timeseries.iter() {
                    if timeseries.time.day() == current_time.day() {
                        output.push_str(&format!(
                            "{}: {} {}°C\n",
                            timeseries.time.format("%H:%M"),
                            format_weather_description(
                                timeseries
                                    .data
                                    .next_1_hours
                                    .as_ref()
                                    .unwrap()
                                    .summary
                                    .symbol_code
                                    .as_str()
                            ),
                            timeseries
                                .data
                                .instant
                                .details
                                .air_temperature
                                .as_ref()
                                .unwrap()
                        ));
                    }
                }
            }
            DurationType::Tomorrow => {
                output.push_str(&format!("on {}\n", current_time.format("%A, %d %B")));
                // select every timeseries that is tomorrow
                // for each timeseries, print the time and the summary
                for timeseries in self.properties.timeseries.iter() {
                    if timeseries.time.day() == current_time.day() + 1 {
                        output.push_str(&format!(
                            "{}: {} {}°C\n",
                            timeseries.time.format("%H:%M"),
                            format_weather_description(
                                timeseries
                                    .data
                                    .next_1_hours
                                    .as_ref()
                                    .unwrap()
                                    .summary
                                    .symbol_code
                                    .as_str()
                            ),
                            timeseries
                                .data
                                .instant
                                .details
                                .air_temperature
                                .as_ref()
                                .unwrap()
                        ));
                    }
                }
            }
            DurationType::Week => {
                output.push_str(&format!("this week\n"));
                // select every timeseries that is this week
                // for each timeseries, print the day, time and the summary
                for timeseries in self.properties.timeseries.iter() {
                    if timeseries.time.day() >= current_time.day()
                        && timeseries.time.day() <= current_time.day() + 7
                    {
                        output.push_str(&format!(
                            "{} {}: {} {}C\n",
                            timeseries.time.format("%A"),
                            timeseries.time.format("%H:%M"),
                            format_weather_description(
                                timeseries
                                    .data
                                    .next_12_hours
                                    .as_ref()
                                    .unwrap()
                                    .summary
                                    .symbol_code
                                    .as_str()
                            ),
                            timeseries
                                .data
                                .instant
                                .details
                                .air_temperature
                                .as_ref()
                                .unwrap()
                        ));
                    }
                }
            }
        }
        Ok(output)
    }
}

fn format_weather_description(description: &str) -> String {
    match description {
        "clearsky_day" => "☀️ Clear Sky (Day)".to_string(),
        "fair_day" => "🌤️ Fair (Day)".to_string(),
        "partlycloudy_day" => "⛅ Partly Cloudy (Day)".to_string(),
        "cloudy" => "☁️ Cloudy".to_string(),
        "rainshowers_day" => "🌦️ Rain Showers (Day)".to_string(),
        "rainshowersandthunder_day" => "⛈️ Rain Showers and Thunder (Day)".to_string(),
        "sleetshowers_day" => "🌨️ Sleet Showers (Day)".to_string(),
        "snowshowers_day" => "❄️ Snow Showers (Day)".to_string(),
        "rain" => "🌧️ Rain".to_string(),
        "heavyrain" => "🌧️ Heavy Rain".to_string(),
        "heavyrainandthunder" => "⛈️ Heavy Rain and Thunder".to_string(),
        "sleet" => "🌨️ Sleet".to_string(),
        "snow" => "❄️ Snow".to_string(),
        "snowandthunder" => "⛈️ Snow and Thunder".to_string(),
        "fog" => "🌫️ Fog".to_string(),
        "sleetshowersandthunder_day" => "⛈️ Sleet Showers and Thunder (Day)".to_string(),
        "snowshowersandthunder_day" => "⛈️ Snow Showers and Thunder (Day)".to_string(),
        "rainandthunder" => "⛈️ Rain and Thunder".to_string(),
        "sleetandthunder" => "⛈️ Sleet and Thunder".to_string(),
        "lightrainshowersandthunder_day" => "⛈️ Light Rain Showers and Thunder (Day)".to_string(),
        "heavyrainshowersandthunder_day" => "⛈️ Heavy Rain Showers and Thunder (Day)".to_string(),
        "lightssleetshowersandthunder_day" => "⛈️ Light Sleet Showers and Thunder (Day)".to_string(),
        "heavysleetshowersandthunder_day" => "⛈️ Heavy Sleet Showers and Thunder (Day)".to_string(),
        "lightssnowshowersandthunder_day" => "⛈️ Light Snow Showers and Thunder (Day)".to_string(),
        "heavysnowshowersandthunder_day" => "⛈️ Heavy Snow Showers and Thunder (Day)".to_string(),
        "lightrainandthunder" => "⛈️ Light Rain and Thunder".to_string(),
        "lightsleetandthunder" => "⛈️ Light Sleet and Thunder".to_string(),
        "heavysleetandthunder" => "⛈️ Heavy Sleet and Thunder".to_string(),
        "lightsnowandthunder" => "⛈️ Light Snow and Thunder".to_string(),
        "heavysnowandthunder" => "⛈️ Heavy Snow and Thunder".to_string(),
        "lightrainshowers_day" => "🌦️ Light Rain Showers (Day)".to_string(),
        "heavyrainshowers_day" => "🌦️ Heavy Rain Showers (Day)".to_string(),
        "lightsleetshowers_day" => "🌦️ Light Sleet Showers (Day)".to_string(),
        "heavysleetshowers_day" => "🌦️ Heavy Sleet Showers (Day)".to_string(),
        "lightsnowshowers_day" => "🌦️ Light Snow Showers (Day)".to_string(),
        "heavysnowshowers_day" => "🌦️ Heavy Snow Showers (Day)".to_string(),
        "lightrain" => "🌧️ Light Rain".to_string(),
        "lightsleet" => "🌨️ Light Sleet".to_string(),
        "heavysleet" => "🌨️ Heavy Sleet".to_string(),
        "lightsnow" => "❄️ Light Snow".to_string(),
        "heavysnow" => "❄️ Heavy Snow".to_string(),
        "clearsky_night" => "🌙 Clear Sky (Night)".to_string(),
        "fair_night" => "🌙 Fair (Night)".to_string(),
        "partlycloudy_night" => "🌙☁️P Partly Cloudy (Night)".to_string(),
        "rainshowers_night" => "🌦️ Rain Showers (Night)".to_string(),
        "rainshowersandthunder_night" => "⛈️ Rain Showers and Thunder (Night)".to_string(),
        "sleetshowers_night" => "🌨️ Sleet Showers (Night)".to_string(),
        "snowshowers_night" => "❄️ Snow Showers (Night)".to_string(),
        "sleetshowersandthunder_night" => "⛈️ Sleet Showers and Thunder (Night)".to_string(),
        "snowshowersandthunder_night" => "⛈️ Snow Showers and Thunder (Night)".to_string(),
        "lightrainshowersandthunder_night" => {
            "⛈️ Light Rain Showers and Thunder (Night)".to_string()
        }
        "heavyrainshowersandthunder_night" => {
            "⛈️ Heavy Rain Showers and Thunder (Night)".to_string()
        }
        "lightssleetshowersandthunder_night" => {
            "⛈️ Light Sleet Showers and Thunder (Night)".to_string()
        }
        "heavysleetshowersandthunder_night" => {
            "⛈️ Heavy Sleet Showers and Thunder (Night)".to_string()
        }
        "lightssnowshowersandthunder_night" => {
            "⛈️ Light Snow Showers and Thunder (Night)".to_string()
        }
        "heavysnowshowersandthunder_night" => {
            "⛈️ Heavy Snow Showers and Thunder (Night)".to_string()
        }
        "lightrainshowers_night" => "🌦️ Light Rain Showers (Night)".to_string(),
        "heavyrainshowers_night" => "🌦️ Heavy Rain Showers (Night)".to_string(),
        "lightsleetshowers_night" => "🌦️ Light Sleet Showers (Night)".to_string(),
        "heavysleetshowers_night" => "🌦️ Heavy Sleet Showers (Night)".to_string(),
        "lightsnowshowers_night" => "🌦️ Light Snow Showers (Night)".to_string(),
        "heavysnowshowers_night" => "🌦️ Heavy Snow Showers (Night)".to_string(),
        "clearsky_polartwilight" => "🌌 Clear Sky (Polar Twilight)".to_string(),
        "fair_polartwilight" => "🌌 Fair (Polar Twilight)".to_string(),
        "partlycloudy_polartwilight" => "🌌 Partly Cloudy (Polar Twilight)".to_string(),
        "rainshowers_polartwilight" => "🌌 Rain Showers (Polar Twilight)".to_string(),
        "rainshowersandthunder_polartwilight" => {
            "🌌 Rain Showers and Thunder (Polar Twilight)".to_string()
        }
        "sleetshowers_polartwilight" => "🌌 Sleet Showers (Polar Twilight)".to_string(),
        "snowshowers_polartwilight" => "🌌 Snow Showers (Polar Twilight)".to_string(),
        "sleetshowersandthunder_polartwilight" => {
            "🌌 Sleet Showers and Thunder (Polar Twilight)".to_string()
        }
        "snowshowersandthunder_polartwilight" => {
            "🌌 Snow Showers and Thunder (Polar Twilight)".to_string()
        }
        "lightrainshowersandthunder_polartwilight" => {
            "🌌 Light Rain Showers and Thunder (Polar Twilight)".to_string()
        }
        "heavyrainshowersandthunder_polartwilight" => {
            "🌌 Heavy Rain Showers and Thunder (Polar Twilight)".to_string()
        }
        "lightssleetshowersandthunder_polartwilight" => {
            "🌌 Light Sleet Showers and Thunder (Polar Twilight)".to_string()
        }
        "heavysleetshowersandthunder_polartwilight" => {
            "🌌 Heavy Sleet Showers and Thunder (Polar Twilight)".to_string()
        }
        "lightssnowshowersandthunder_polartwilight" => {
            "🌌 Light Snow Showers and Thunder (Polar Twilight)".to_string()
        }
        "heavysnowshowersandthunder_polartwilight" => {
            "🌌 Heavy Snow Showers and Thunder (Polar Twilight)".to_string()
        }
        "lightrainshowers_polartwilight" => "🌌 Light Rain Showers (Polar Twilight)".to_string(),
        "heavyrainshowers_polartwilight" => "🌌 Heavy Rain Showers (Polar Twilight)".to_string(),
        "lightsleetshowers_polartwilight" => "🌌 Light Sleet Showers (Polar Twilight)".to_string(),
        "heavysleetshowers_polartwilight" => "🌌 Heavy Sleet Showers (Polar Twilight)".to_string(),
        "lightsnowshowers_polartwilight" => "🌌 Light Snow Showers (Polar Twilight)".to_string(),
        "heavysnowshowers_polartwilight" => "🌌 Heavy Snow Showers (Polar Twilight)".to_string(),
        _ => description.to_string(), // Default to the original description if not found
    }
}
