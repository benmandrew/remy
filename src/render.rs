use crate::state::{EntryWithAuthor, State};
use ratatui::prelude::*;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use scraper::Html;

fn entry_to_list_item<'a>(entry: &'a EntryWithAuthor) -> ListItem<'a> {
    let content = entry
        .entry
        .title
        .as_ref()
        .map(|t| t.content.as_str())
        .unwrap_or("No Title");
    let updated = entry
        .entry
        .updated
        .map(|d| d.format("%d/%m/%Y").to_string())
        .unwrap_or_else(|| "Unknown Date".to_string());
    let mut display_text = Text::from(content);
    display_text.push_line(
        Line::from(format!("{} - {}", entry.author, updated))
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

struct TextSpan {
    text: String,
    tags: Vec<String>,
}

fn document_to_text(document: &str) -> Vec<TextSpan> {
    let html = Html::parse_document(document);
    let mut spans = Vec::new();
    let mut stack = Vec::new();
    fn traverse(
        element_ref: scraper::ElementRef,
        stack: &mut Vec<String>,
        spans: &mut Vec<TextSpan>,
    ) {
        stack.push(element_ref.value().name().to_string());
        for child in element_ref.children() {
            match child.value() {
                scraper::node::Node::Element(_) => {
                    if let Some(child_ref) = scraper::ElementRef::wrap(child) {
                        traverse(child_ref, stack, spans);
                    }
                }
                scraper::node::Node::Text(text) => {
                    let txt = text.text.to_string();
                    if !txt.is_empty() {
                        spans.push(TextSpan {
                            text: txt.to_string(),
                            tags: stack.clone(),
                        });
                    }
                }
                _ => {}
            }
        }
        stack.pop();
    }
    traverse(html.root_element(), &mut stack, &mut spans);
    spans
}

fn render_spans(textspans: Vec<TextSpan>) -> Vec<Line<'static>> {
    let mut lines = Vec::<Line>::new();
    let mut spans = Vec::<Span>::new();
    for textspan in textspans {
        let mut span = Span::from(textspan.text.clone());
        if textspan.tags.contains(&"a".to_string()) {
            span = span.underlined().fg(Color::Blue)
        } else if textspan.tags.contains(&"strong".to_string())
            || textspan.tags.contains(&"b".to_string())
        {
            span = span.bold()
        } else if textspan.tags.contains(&"em".to_string())
            || textspan.tags.contains(&"i".to_string())
        {
            span = span.italic()
        } else if textspan.tags.contains(&"h1".to_string())
            || textspan.tags.contains(&"h2".to_string())
            || textspan.tags.contains(&"h3".to_string())
        {
            span = span.bold().fg(Color::Yellow)
        } else if textspan.tags.contains(&"pre".to_string()) {
            span = span.italic().fg(Color::Green)
        }
        spans.push(span);
        if textspan.tags.ends_with(&["p".to_string()])
            || textspan.tags.ends_with(&["div".to_string()])
            || textspan.tags.ends_with(&["br".to_string()])
            || textspan.tags.ends_with(&["h1".to_string()])
            || textspan.tags.ends_with(&["h2".to_string()])
            || textspan.tags.ends_with(&["h3".to_string()])
            || textspan.tags.ends_with(&["li".to_string()])
        {
            lines.push(Line::from(spans.clone()));
            spans.clear();
        }
    }
    if !spans.is_empty() {
        lines.push(Line::from(spans));
    }
    lines
}

fn render_selected_entry(frame: &mut Frame, area: Rect, entry_body: &str) {
    let entry_body = document_to_text(entry_body);
    let lines = render_spans(entry_body);
    let paragraph = Paragraph::new(lines)
        .block(Block::new().borders(Borders::ALL))
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn render_selected_entry_raw(frame: &mut Frame, area: Rect, entry_body: &str) {
    let paragraph = Paragraph::new(entry_body.to_string())
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
    if state.render_raw_html {
        render_selected_entry_raw(
            frame,
            layout[1],
            state.get_selected_entry_body(),
        );
    } else {
        render_selected_entry(
            frame,
            layout[1],
            state.get_selected_entry_body(),
        );
    }
}
