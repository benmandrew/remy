mod feed;
mod render;
mod state;

use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;

use crate::render::render;

fn run(mut terminal: DefaultTerminal, state: &mut state::State) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|f| render(f, state))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down => state.move_down(),
                KeyCode::Up => state.move_up(),
                _ => break Ok(()),
            }
        }
    }
}

fn get_state() -> Result<state::State, std::io::Error> {
    match feed::get("feeds.txt") {
        Ok(feeds) => Ok(state::State::new(feeds)),
        Err(e) => Err(e),
    }
}

fn main() -> Result<(), std::io::Error> {
    let terminal = ratatui::init();
    let result = match get_state() {
        Ok(mut s) => run(terminal, &mut s),
        Err(e) => Err(e),
    };
    ratatui::restore();
    result
}
