pub struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub data: Vec<(&'a str, u64)>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            titles: vec!["Tab0", "Tab1", "Tab2", "Tab3"], //app struct holds an array for titles to different tabs
            index: 0, //app struct holds an index variable to keep track of where the user is in the array
            data: vec![ ("Rain", 5), ("Sleet", 7), ("Snow", 15)],
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) { //similar function, if else for the same wrapping effect
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}