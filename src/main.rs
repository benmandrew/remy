mod render;
mod state;

use crate::state::State;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;

use crate::render::render;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut state = State::new(vec!["Item 1", "Item 2", "Item 3", "Item 4"]);
    let result = run(terminal, &mut state);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, state: &mut state::State) -> Result<()> {
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
