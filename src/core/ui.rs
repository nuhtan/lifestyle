use std::{io::{self, Error}, time::Duration};

use crossterm::{event::{self, Event, KeyCode, poll}, terminal::enable_raw_mode};
use tui::{Terminal, backend::CrosstermBackend, widgets::{Block, Borders}};

pub fn initialize() -> Result<(), Error> {
    let backend = CrosstermBackend::new(io::stdout());
    enable_raw_mode().unwrap();
    let mut terminal = Terminal::new(backend)?;
    loop {
        let size;
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Blocky").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;
        
        if poll(Duration::from_millis(1_000)).unwrap() {
            let event = event::read().unwrap();
            match event {
                Event::Key(KeyCode::Esc) => break,

                _ => {}
            }
        }
    }
    Ok(())
}