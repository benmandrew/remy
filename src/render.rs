use crate::state::State;
use feed_rs::model::Entry;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

fn render_entry_list(
    frame: &mut Frame,
    area: Rect,
    items: &[&str],
    state: &mut ListState,
) {
    let list_items: Vec<ListItem> =
        items.iter().map(|i| ListItem::new(*i)).collect();
    let list = List::new(list_items)
        .block(Block::new().borders(Borders::ALL).title("Feeds"))
        .highlight_style(Style::new().reversed());
    frame.render_stateful_widget(list, area, state)
}

fn get_entry_body(entry: &Entry) -> &str {
    entry
        .content
        .as_ref()
        .and_then(|c| c.body.as_deref())
        .unwrap_or("No Content")
}

fn render_selected_entry(frame: &mut Frame, area: Rect, entry: &Entry) {
    let paragraph = Paragraph::new(get_entry_body(entry))
        .block(Block::new().borders(Borders::ALL));
    frame.render_widget(paragraph, area);
}

pub fn render(frame: &mut Frame, state: &mut State) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.area());
    render_entry_list(
        frame,
        layout[0],
        &state
            .entries
            .iter()
            .map(|e| {
                e.title
                    .as_ref()
                    .map(|t| t.content.as_str())
                    .unwrap_or("No Title")
            })
            .collect::<Vec<&str>>(),
        &mut state.list_state,
    );
    render_selected_entry(frame, layout[1], &state.entries[state.selected]);
}
