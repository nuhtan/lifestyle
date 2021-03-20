use crate::core::{state_data::state::State, UIState};

use std::{
    io::{self, Error},
    time::Duration,
};

use crossterm::{
    event::{self, poll, Event, KeyCode},
    terminal::enable_raw_mode,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols::Marker,
    text::{Span, Spans},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem, Paragraph, Tabs},
    Terminal,
};

pub fn run(app_state: State) -> Result<(), Error> {
    let backend = CrosstermBackend::new(io::stdout());
    enable_raw_mode().unwrap();
    let mut terminal = Terminal::new(backend)?;
    let mut ui_state = UIState::new(vec!["Overview", "Calories", "Temp"]);
    terminal.clear()?;
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.size());
            let titles = ui_state
                .tabs
                .titles
                .iter()
                .map(|t| Spans::from(vec![Span::styled(*t, Style::default().fg(Color::White))]))
                .collect();
            let tabs = Tabs::new(titles)
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(ui_state.tabs.index)
                .style(Style::default().fg(Color::Yellow))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::LightYellow),
                );
            f.render_widget(tabs, chunks[0]);
            match ui_state.tabs.index {
                0 => {
                    let interior_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(40), Constraint::Percentage(60)].as_ref(),
                        )
                        .split(chunks[1]);
                    let block = Paragraph::new(format!(
                        "Server is running on http://{}.{}.{}.{}:{}.",
                        app_state.addr[0],
                        app_state.addr[1],
                        app_state.addr[2],
                        app_state.addr[3],
                        app_state.port
                    ))
                    .block(
                        Block::default()
                            .title("General Information")
                            .borders(Borders::ALL),
                    );
                    let items: Vec<ListItem> = app_state
                        .requests
                        .lock()
                        .unwrap()
                        .items
                        .iter()
                        .map(|i| ListItem::new(i.clone()))
                        .collect();
                    let requests = List::new(items)
                        .block(Block::default().title("Requests").borders(Borders::ALL))
                        .highlight_symbol("> ");
                    f.render_widget(block, interior_chunks[0]);
                    f.render_stateful_widget(
                        requests,
                        interior_chunks[1],
                        &mut app_state.requests.lock().unwrap().state,
                    );
                }
                1 => {
                    // Generate data sets based on the calorie data. There should be 3 sets of data, the total calorie intake and the total burn are scatter plots and the combined calories for the day is a line graph.
                    let calories = app_state.calories.lock().unwrap().clone();
                    let mut combined_data: Vec<(f64, f64)> = Vec::new();
                    let mut consume_data: Vec<(f64, f64)> = Vec::new();
                    let mut burn_data: Vec<(f64, f64)> = Vec::new();
                    let calories_len = calories.len();

                    for cal in calories {
                        combined_data.push(((cal.index) as f64, cal.total as f64));
                        consume_data.push(((cal.index) as f64, (cal.total + cal.burn) as f64));
                        burn_data.push(((cal.index) as f64, cal.burn as f64));
                    }

                    let combined = Dataset::default()
                        .name("Combined")
                        .marker(Marker::Braille)
                        .graph_type(GraphType::Line)
                        .style(Style::default().fg(Color::Green))
                        .data(combined_data.as_slice());
                    let consumed = Dataset::default()
                        .name("Consumed")
                        .marker(Marker::Dot)
                        .graph_type(GraphType::Scatter)
                        .style(Style::default().fg(Color::Red))
                        .data(consume_data.as_slice());
                    let burned = Dataset::default()
                        .name("Burn")
                        .marker(Marker::Dot)
                        .graph_type(GraphType::Scatter)
                        .style(Style::default().fg(Color::Cyan))
                        .data(burn_data.as_slice());
                    let datasets = vec![combined, consumed, burned];
                    let graphs = Chart::new(datasets)
                        .block(Block::default().title("Calories").borders(Borders::ALL))
                        .x_axis(
                            Axis::default()
                                .title(Span::styled(
                                    "Days",
                                    Style::default().fg(Color::LightYellow),
                                ))
                                .style(Style::default().fg(Color::White))
                                .bounds([1.0, (calories_len) as f64])
                                .labels(
                                    [
                                        String::from("1"),
                                        format!("{}", calories_len),
                                    ]
                                    .iter()
                                    .cloned()
                                    .map(Span::from)
                                    .collect(),
                                ),
                        )
                        .y_axis(
                            Axis::default()
                                .title(Span::styled(
                                    "Calories",
                                    Style::default().fg(Color::LightYellow),
                                ))
                                .style(Style::default().fg(Color::White))
                                .bounds([0.0, 3000.0])
                                .labels(["0", "3000"].iter().cloned().map(Span::from).collect()),
                        );
                    f.render_widget(graphs, chunks[1]);
                }
                2 => {
                    let block = Block::default().title("Inner 2").borders(Borders::ALL);
                    f.render_widget(block, chunks[1]);
                }
                _ => unreachable!(),
            };
        })?;

        if poll(Duration::from_millis(1_000)).unwrap() {
            let event = event::read().unwrap();
            match event {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Esc => break, // Ends the ui if the escape key is pressed
                        KeyCode::Right => ui_state.tabs.next(),
                        KeyCode::Left => ui_state.tabs.previous(),
                        KeyCode::Down => match ui_state.tabs.index {
                            0 => app_state.requests.lock().unwrap().next(),
                            _ => {}
                        },
                        KeyCode::Up => match ui_state.tabs.index {
                            0 => app_state.requests.lock().unwrap().previous(),
                            _ => {}
                        },
                        _ => {} // Other keys are ignored
                    }
                }
                _ => {} // Other events are ignored
            }
        }
    }
    terminal.clear()?;
    Ok(())
}
