use crate::terminal::my_function;

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
            weather_string: my_function().unwrap().to_string(),
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}