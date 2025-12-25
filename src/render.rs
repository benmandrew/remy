use crate::state::State;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

fn render_feeds_list(frame: &mut Frame, area: Rect, items: &[&str], state: &mut ListState) {
    let list_items: Vec<ListItem> = items.iter().map(|i| ListItem::new(*i)).collect();

    let list = List::new(list_items)
        .block(Block::new().borders(Borders::ALL).title("Feeds"))
        .highlight_style(Style::new().reversed());
    frame.render_stateful_widget(list, area, state);
}

pub fn render(frame: &mut Frame, state: &mut State) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());
    render_feeds_list(
        frame,
        layout[0],
        &state
            .feeds
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>(),
        &mut state.list_state,
    );

    frame.render_widget(
        Paragraph::new("Right Side").block(Block::new().borders(Borders::ALL)),
        layout[1],
    );
}
