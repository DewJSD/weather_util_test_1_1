/* Names: Garrett Thompson, Jeffery Dew
 * Date: 11/8/23
 * Description: File that configures the app's UI and calls the draw function
 */

pub mod app;

use ratatui::{prelude::*, widgets::*};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use std::error::Error;

use crate::terminal::app::App;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Right => app.next(),
                    KeyCode::Left => app.previous(),
                    _ => {}
                }
            }
        }
    }
}

fn ui(frame: &mut Frame, app: &App) {
    let b = Block::default()
        .title(block::Title::from("-Y-O-U-R-").alignment(Alignment::Left))
        .title(block::Title::from("-W-E-A-T-H-E-R-").alignment(Alignment::Center))
        .title(block::Title::from("-R-E-P-O-R-T-").alignment(Alignment::Right))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![first.yellow(), rest.white()])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.index)
        .style(Style::default().white().on_blue())
        .highlight_style(Style::default().red().on_white())
        .divider(symbols::DOT);

    let barchart = BarChart::default()
        .block(Block::default().title("Weather Data").borders(Borders::ALL))
        .data(&app.data)
        .bar_width(18)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));

    let weather_paragraph = Paragraph::new(app.weather_string.clone());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.size());

    frame.render_widget(tabs, chunks[0]);

    //depending on the value of index, this changes what is displayed, notice you can define a block and then add it to a paragraph

    match app.index {
        0 => frame.render_widget(
            weather_paragraph.block(b).alignment(Alignment::Center),
            chunks[1],
        ),
        1 => frame.render_widget(barchart, chunks[1]),
        _ => unreachable!(),
    };
}
