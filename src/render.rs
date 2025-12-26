use crate::state::State;
use feed_rs::model::Entry;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use scraper::Html;

fn entry_to_list_item<'a>(entry: &'a Entry) -> ListItem<'a> {
    let content = entry
        .title
        .as_ref()
        .map(|t| t.content.as_str())
        .unwrap_or("No Title");
    let author = entry
        .authors
        .first()
        .map(|a| a.name.as_str())
        .unwrap_or("Unknown Author");
    let updated = entry
        .updated
        .map(|d| d.format("%d/%m/%Y").to_string())
        .unwrap_or_else(|| "Unknown Date".to_string());
    let mut display_text = Text::from(content);
    display_text.push_line(
        Line::from(format!("{} - {}", author, updated))
            .italic()
            .right_aligned(),
    );
    ListItem::new(display_text)
}

fn render_entry_list(frame: &mut Frame, area: Rect, state: &mut State) {
    let list_items: Vec<ListItem> =
        state.entries.iter().map(entry_to_list_item).collect();
    let list = List::new(list_items)
        .block(Block::new().borders(Borders::ALL))
        .highlight_style(Style::new().reversed());
    frame.render_stateful_widget(list, area, &mut state.list_state);
}

fn document_to_text(document: &str) -> String {
    let html = Html::parse_document(document);
    let mut output = String::with_capacity(document.len());
    for value in html.tree.values() {
        if let scraper::node::Node::Text(text) = &value {
            output.push_str(&text.text);
        }
    }
    output
}

fn render_selected_entry(frame: &mut Frame, area: Rect, entry_body: &str) {
    let entry_body = document_to_text(entry_body);
    let paragraph = Paragraph::new(entry_body)
        .block(Block::new().borders(Borders::ALL))
        .wrap(Wrap { trim: false });
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
    render_entry_list(frame, layout[0], state);
    render_selected_entry(frame, layout[1], state.get_selected_entry_body());
}
