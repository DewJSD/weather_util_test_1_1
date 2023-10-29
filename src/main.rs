/* Names: Garrett Thompson, Jeffery Dew
 * Date: 10/29/23
 * Description: Terminal application that draws a user interface and displays data retrieved from a weather API
 */


mod terminal;

use terminal::{
    *,
    app::App,
};
/* LOADING CRATES */

//crate that provides functions and structs to link application to openweathermap.org
use weather_util_rust::{
    weather_api::{WeatherApi, WeatherLocation},
};

//accessing the standard library
use std::{
    path::Path,
    ops::Deref,
    env,
    io::*,
    cell::RefCell,
};

//crate for reading environment variables from a .env file
use dotenvy::*;

//interprets user inputs
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    ExecutableCommand, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

//crate that provides methods for creating a terminal user interface
use ratatui::{prelude::*, widgets::*};


fn main(){
    enable_raw_mode(); //lines 45-50 are essential for ratatui to run
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear();

    let app = App::new(); //creation of the app
    let _res = run_app(&mut terminal, app);

    //lines 58-64 are essential for ratatui to run
    disable_raw_mode();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
}
