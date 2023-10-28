use clap::Parser;
use geocoding::{get_cooordinates, get_display_name};

pub mod args;
pub mod geocoding;
pub mod weather;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = args::WeatherArgs::parse();

    let coords = &(geocoding::get_location_data(&args).await?.unwrap()[0]);

    let weather_data = weather::get_weather_data(get_cooordinates(&coords))
        .await?
        .unwrap();

    let _ = match weather_data.display(
        args.duration.unwrap(),
        get_display_name(&coords),
        args.output_mode.unwrap(),
    ) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    };

    Ok(())
}
