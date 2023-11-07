use weather_util_rust::{
    StringType,
    weather_data::*,
    weather_api::{WeatherApi, WeatherLocation},
    weather_forecast::{WeatherForecast, ForecastEntry, CityEntry},
};

use time::{Duration, format_description::FormatItem, Weekday};
use time::format_description::modifier::Padding;
use time::macros::format_description;
use time::OffsetDateTime;

use tokio::main;

pub struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub data: Vec<(&'a str, u64)>,
    pub weather_string: String,
    pub zipcode: WeatherLocation,
}


impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            titles: vec!["Tab0", "Tab1", "Tab2"], //app struct holds an array for titles to different tabs
            index: 0, //app struct holds an index variable to keep track of where the user is in the array
            data: vec![],
            weather_string: String::new(),
            zipcode: WeatherLocation::from_zipcode(57107),
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
        self.weather_string = self.my_function().unwrap().to_string();
        self.load_forecast();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
        self.weather_string = self.my_function().unwrap().to_string();
        self.load_forecast();
    }

    #[tokio::main]

        pub async fn my_function(&self) -> Result<StringType, Box<dyn std::error::Error>> {
        let api = Self::set_api().await?;
        let data:WeatherData = api.get_weather_data(&self.zipcode).await?;
        let w3 = data.get_current_conditions();
        return Ok(w3);
    }

    pub async fn load_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let api = Self::set_api().await?;
        let data:WeatherData = api.get_weather_data(&self.zipcode).await?;
        let msg = String::new();
        return Ok(msg);
    }

    pub async fn set_api() -> Result<WeatherApi, Box<dyn std::error::Error>> {
        let api_key = "f1e875bd567884ff618ff3c7bb8d6e19";
        let api_endpoint = "api.openweathermap.org";
        let api_path = "data/2.5/";
        let geo_path = "geo/1.0/";
        let api = WeatherApi::new(&api_key, &api_endpoint, &api_path, &geo_path);
        Ok(api)
    }
    
    #[tokio::main]

    pub async fn load_forecast(&mut self){
        self.data.clear();
        let location = WeatherLocation::from_zipcode(57107);
        let api = Self::set_api().await.unwrap();
        let forecast: WeatherForecast = api.get_weather_forecast(&location).await.unwrap();

        for (index, entry) in forecast.list.iter().enumerate() {

            // Access the temperature from the current entry
            let temp_fahrenheit: f64 = entry.main.temp.fahrenheit();
            // Create a tuple with the day of the week and temperature
            

            let data = ("City", temp_fahrenheit as u64);
            
            // Push the tuple to the app.data vector
            self.data.push(data);
    }
}

}