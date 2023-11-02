use weather_util_rust::{
    StringType,
    weather_data::*,
    weather_api::{WeatherApi, WeatherLocation},
};


pub struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub data: Vec<(&'a str, u64)>,
    pub weather_string: String,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            titles: vec!["Tab0", "Tab1", "Tab2"], //app struct holds an array for titles to different tabs
            index: 0, //app struct holds an index variable to keep track of where the user is in the array
            data: vec![ ("Rain", 5), ("Sleet", 7), ("Snow", 15)],
            weather_string: String::new(),
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
        self.weather_string = self.my_function().unwrap().to_string();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
        self.weather_string = self.my_function().unwrap().to_string();
    }

    #[tokio::main]

        pub async fn my_function(&self) -> Result<StringType, Box<dyn std::error::Error>> {
let api = Self::set_api().await?;
        let zipcode_location = WeatherLocation::from_zipcode(73301);
        let data:WeatherData = api.get_weather_data(&zipcode_location).await?;
        let w3 = data.get_current_conditions();
        return Ok(w3);
    }

    pub async fn set_api() -> Result<WeatherApi, Box<dyn std::error::Error>> {
        let api_key = "f1e875bd567884ff618ff3c7bb8d6e19";
        let api_endpoint = "api.openweathermap.org";
        let api_path = "data/2.5/";
        let geo_path = "geo/1.0/";
        let api = WeatherApi::new(&api_key, &api_endpoint, &api_path, &geo_path);
        Ok(api)
    }

}