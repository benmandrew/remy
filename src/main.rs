mod feed;
mod popup;
mod render;
mod state;

use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;
use std::fs::File;
use tokio::sync::mpsc;

use crate::render::render;

const LOG_PATH: &str = "remy.log";

fn init_logger() {
    simplelog::WriteLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        File::create(LOG_PATH).unwrap(),
    )
    .unwrap();
}

fn init_crossterm() {
    crossterm::execute!(
        std::io::stdout(),
        crossterm::event::EnableMouseCapture
    )
    .unwrap();
}

fn handle_key_event(event: event::KeyEvent, state: &mut state::State) -> bool {
    let mut exit = false;
    match event.code {
        KeyCode::Down => state.move_down(),
        KeyCode::Up => state.move_up(),
        KeyCode::Left => state.move_left(),
        KeyCode::Right => state.move_right(),
        KeyCode::Enter => state.open_selected_entry_link(),
        KeyCode::Char('r') => state.switch_render_mode(),
        KeyCode::Char('h') => match state.selected_window {
            state::SelectedWindow::HelpPopup => {
                state.selected_window = state::SelectedWindow::EntryList;
            }
            _ => {
                state.selected_window = state::SelectedWindow::HelpPopup;
            }
        },
        KeyCode::Char('q') => exit = true,
        _ => {}
    };
    exit
}

fn handle_mouse_event(
    event: event::MouseEvent,
    terminal: &DefaultTerminal,
    state: &mut state::State,
) -> Result<(), std::io::Error> {
    log::warn!("Bruh");
    if event.kind == event::MouseEventKind::Down(event::MouseButton::Left)
        && state
            .separator
            .mouse_on_separator(event.column, terminal.size()?.width)
    {
        state.separator.mouse_down = true;
    } else if event.kind == event::MouseEventKind::Up(event::MouseButton::Left)
    {
        state.separator.mouse_down = false;
    }
    if state.separator.mouse_down
        && event.kind == event::MouseEventKind::Drag(event::MouseButton::Left)
    {
        state
            .separator
            .update_position(event.column, terminal.size()?.width);
    }
    Ok(())
}

fn run(
    mut terminal: DefaultTerminal,
    state: &mut state::State,
    mut feed_rx: mpsc::Receiver<(Vec<feed_rs::model::Feed>, Vec<String>)>,
) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|f| render(f, state))?;
        if let Ok((feeds, urls)) = feed_rx.try_recv() {
            state.update_feeds(feeds.clone());
            tokio::spawn(async move {
                let _ = feed::save_cached_feeds(&feeds, &urls).await;
            });
        }
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(event) => {
                    if handle_key_event(event, state) {
                        break Ok(());
                    }
                }
                Event::Mouse(event) => {
                    handle_mouse_event(event, &terminal, state)?;
                }
                _ => {}
            }
        }
    }
}

const FEED_PATH: &str = "feeds.txt";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    init_logger();
    init_crossterm();
    let (feed_tx, feed_rx) = mpsc::channel(1);
    let initial_feeds = feed::load_cached_feeds().await.unwrap_or_default();
    let mut state = state::State::new(initial_feeds);
    let feed_tx_clone = feed_tx.clone();
    tokio::spawn(async move {
        if let Ok((feeds, urls)) = feed::get(FEED_PATH).await {
            let _ = feed_tx_clone.send((feeds, urls)).await;
        }
    });
    let terminal = ratatui::init();
    let result = run(terminal, &mut state, feed_rx);
    ratatui::restore();
    result
}
