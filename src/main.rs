use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());
    frame.render_widget(
        Paragraph::new("Left").block(
            Block::new()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        ),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new("Right").block(
            Block::new()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        ),
        layout[1],
    );
}
