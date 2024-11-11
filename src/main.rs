
use reqwest::Error;
use serde::Deserialize;
use std::env;

const API_KEY: &str = "your api key here";

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: u8,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: weather_cli <CITY_NAME>");
        return Ok(());
    }
    let city_name = &args[1];
    

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city_name, API_KEY
    );
    let response = reqwest::get(&url)
        .await?
        .json::<WeatherResponse>()
        .await?;
    
    println!("Weather in  {}:", city_name);
    println!("Temperature: {}°C", response.main.temp);
    println!("Humidity: {}%", response.main.humidity);
    if let Some(weather) = response.weather.first() {
        println!("Condition: {}", weather.description);
    }

    Ok(())

}    

