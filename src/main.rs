/* Names: Garrett Thompson, Jeffery Dew
 * Date: 10/29/23
 * Description: Terminal application that draws a user interface and displays data retrieved from a weather API
 */


mod terminal;

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


fn main() {
    println!("Hello, world!");
}
