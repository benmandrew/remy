use crate::state::State;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

fn render_entry_list(
    frame: &mut Frame,
    area: Rect,
    items: &[String],
    state: &mut ListState,
) {
    let list_items: Vec<ListItem> =
        items.iter().map(|i| ListItem::new(i.as_str())).collect();
    let list = List::new(list_items)
        .block(Block::new().borders(Borders::ALL).title("Feeds"))
        .highlight_style(Style::new().reversed());
    frame.render_stateful_widget(list, area, state)
}

fn render_selected_entry(frame: &mut Frame, area: Rect, entry_body: &str) {
    let paragraph =
        Paragraph::new(entry_body).block(Block::new().borders(Borders::ALL));
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
    let entry_titles = state.get_entry_titles();
    render_entry_list(frame, layout[0], &entry_titles, &mut state.list_state);
    render_selected_entry(frame, layout[1], state.get_selected_entry_body());
}
