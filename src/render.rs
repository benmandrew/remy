use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

fn render_list(frame: &mut Frame, area: Rect, items: &[&str]) {
    let list_items: Vec<ListItem> = items.iter().map(|i| ListItem::new(*i)).collect();

    let list = List::new(list_items).block(Block::new().borders(Borders::ALL).title("Items"));

    frame.render_widget(list, area);
}

pub fn render(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    let items = vec!["Item 1", "Item 2", "Item 3", "Item 4"];
    render_list(frame, layout[0], &items);

    frame.render_widget(
        Paragraph::new("Right Side").block(Block::new().borders(Borders::ALL).title("Right Panel")),
        layout[1],
    );
}
