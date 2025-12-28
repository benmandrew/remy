mod feed;
mod render;
mod state;

use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;

use crate::render::render;

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
        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Down => state.move_down(),
                KeyCode::Up => state.move_up(),
                KeyCode::Char('r') => state.switch_render_mode(),
                _ => break Ok(()),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let (feed_tx, feed_rx) = mpsc::channel(1);
    let initial_feeds = feed::load_cached_feeds().await.unwrap_or_default();
    let mut state = state::State::new(initial_feeds);
    let feed_tx_clone = feed_tx.clone();
    tokio::spawn(async move {
        if let Ok((feeds, urls)) = feed::get("feeds.txt").await {
            let _ = feed_tx_clone.send((feeds, urls)).await;
        }
    });
    let terminal = ratatui::init();
    let result = run(terminal, &mut state, feed_rx);
    ratatui::restore();
    result
}
