pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
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

    let titles = app.titles.iter().map(|t| 
        {
            let (first, rest) = t.split_at(1);
            Line::from(vec![first.yellow(), rest.white()])
        })
    .collect();

    let tabs = Tabs::new(titles).block(Block::default().borders(Borders::ALL).title("Tabs"))
    .select(app.index)
    .style(Style::default().white().on_blue())
    .highlight_style(Style::default().red().on_white())
    .divider(symbols::DOT);

    

    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(3), Constraint::Min(0)])
    .split(frame.size());

    frame.render_widget(tabs, chunks[0]);

//depending on the value of index, this changes what is displayed, notice you can define a block and then add it to a paragraph
//I'd like to add a graph to this, I don't know how difficult that will be

    let inner = match app.index {
        0 => Paragraph::new("first").block(b),
        1 => Paragraph::new("second").block(Block::default().title("inner 1").borders(Borders::ALL)),
        2 => Paragraph::new("third").block(b),
        3 => Paragraph::new("fourth").block(Block::default().title("inner 3").borders(Borders::ALL)),
        _ => unreachable!(),
    };

    frame.render_widget(inner, chunks[1]);
}