/* Names: Garrett Thompson, Jeffery Dew
 * Date: 10/29/23
 * Description: Terminal application that draws a user interface and displays data retrieved from a weather API
 */

mod terminal;

use terminal::{app::App, *};
/* LOADING CRATES */

//accessing the standard library
use std::io::*;

//interprets user inputs
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

//crate that provides methods for creating a terminal user interface
use ratatui::prelude::*;

fn main() {
    let _ = enable_raw_mode(); //lines 45-50 are essential for ratatui to run
    let mut stdout = stdout();
    let _ = execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let _ = terminal.clear();

    let mut app = App::new(); //creation of the app
    app.weather_string = app.load_data().unwrap().to_string();
    app.load_forecast_weekly();
    let _ = run_app(&mut terminal, app);

    //lines 58-64 are essential for ratatui to run
    let _ = disable_raw_mode();
    let _ = execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
}
