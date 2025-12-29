use crate::state::{EntryWithAuthor, SelectedWindow, State};
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
    let mut block = Block::new().borders(Borders::ALL);
    if state.selected_window == SelectedWindow::EntryList {
        block = block.border_style(Style::new().blue());
    }
    let list = List::new(list_items)
        .block(block)
        .highlight_style(Style::new().reversed());
    frame.render_stateful_widget(list, area, &mut state.list_state);
}

#[derive(Debug, Clone)]
struct StyledText {
    text: String,
    bold: bool,
    italic: bool,
    underline: bool,
    color: Option<Color>,
    in_pre: bool,
}

impl StyledText {
    fn new(text: String) -> Self {
        Self {
            text,
            bold: false,
            italic: false,
            underline: false,
            color: None,
            in_pre: false,
        }
    }

    fn to_span(&self) -> Span<'static> {
        let mut span = Span::from(self.text.clone());
        if self.bold {
            span = span.bold();
        }
        if self.italic {
            span = span.italic();
        }
        if self.underline {
            span = span.underlined();
        }
        if let Some(color) = self.color {
            span = span.fg(color);
        }
        span
    }
}

fn is_block_element(tag: &str) -> bool {
    matches!(
        tag,
        "p" | "div"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "ul"
            | "ol"
            | "li"
            | "blockquote"
            | "pre"
            | "section"
            | "article"
            | "header"
            | "footer"
            | "nav"
    )
}

fn is_whitespace_preserved(tags: &[String]) -> bool {
    tags.iter().any(|t| t == "pre" || t == "code")
}

fn normalize_whitespace(text: &str, preserve: bool) -> String {
    if preserve {
        text.to_string()
    } else {
        let mut result = String::new();
        let mut prev_was_space = false;
        let leading_space = text.starts_with([' ', '\n', '\t']);
        let trailing_space = text.ends_with([' ', '\n', '\t']);

        for (i, ch) in text.chars().enumerate() {
            if ch == '\n' || ch == '\t' {
                if !prev_was_space && (i > 0 || leading_space) {
                    result.push(' ');
                    prev_was_space = true;
                }
            } else if ch == ' ' {
                result.push(' ');
                prev_was_space = true;
            } else {
                result.push(ch);
                prev_was_space = false;
            }
        }

        if trailing_space && !result.ends_with(' ') && !result.is_empty() {
            result.push(' ');
        }

        result
    }
}

fn apply_tag_styles(styled: &mut StyledText, tag: &str) {
    match tag {
        "b" | "strong" => styled.bold = true,
        "i" | "em" => styled.italic = true,
        "u" => styled.underline = true,
        "pre" | "code" => {
            styled.in_pre = true;
            styled.color = Some(Color::Green);
        }
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
            styled.bold = true;
            styled.color = Some(Color::Yellow);
        }
        "a" => {
            styled.color = Some(Color::Blue);
            styled.underline = true;
        }
        _ => {}
    }
}

fn flush_current_line(
    current_line: &mut Vec<StyledText>,
    lines: &mut Vec<Line<'static>>,
) {
    if !current_line.is_empty() {
        let spans: Vec<Span> =
            current_line.iter().map(|st| st.to_span()).collect();
        lines.push(Line::from(spans));
        current_line.clear();
    }
}

fn add_list_prefix(current_line: &mut Vec<StyledText>, list_depth: usize) {
    let indent = "  ".repeat(list_depth);
    current_line.push(StyledText::new(format!("{}• ", indent)));
}

fn should_add_spacing_after(tag_name: &str) -> bool {
    matches!(tag_name, "p" | "h1" | "h2" | "h3" | "pre" | "blockquote")
}

fn process_text_node(
    text: &str,
    tag_stack: &[String],
    current_line: &mut Vec<StyledText>,
    lines: &mut Vec<Line<'static>>,
) {
    let preserve = is_whitespace_preserved(tag_stack);
    if preserve {
        for line_text in text.split('\n') {
            if !line_text.is_empty() || text.contains('\n') {
                let mut styled = StyledText::new(line_text.to_string());
                for tag in tag_stack.iter() {
                    apply_tag_styles(&mut styled, tag);
                }
                current_line.push(styled);
            }
            if text.contains('\n') {
                flush_current_line(current_line, lines);
            }
        }
    } else {
        let normalized = normalize_whitespace(text, preserve);
        if !normalized.is_empty() {
            let mut styled = StyledText::new(normalized);
            for tag in tag_stack.iter() {
                apply_tag_styles(&mut styled, tag);
            }
            current_line.push(styled);
        }
    }
}

fn handle_block_opening(
    is_block: bool,
    current_line: &mut Vec<StyledText>,
    lines: &mut Vec<Line<'static>>,
) {
    if is_block {
        flush_current_line(current_line, lines);
    }
}

fn handle_block_closing(
    tag_name: &str,
    is_block: bool,
    current_line: &mut Vec<StyledText>,
    lines: &mut Vec<Line<'static>>,
) {
    if is_block {
        flush_current_line(current_line, lines);
        if should_add_spacing_after(tag_name) {
            lines.push(Line::from(""));
        }
    }
}

fn traverse_element(
    element_ref: scraper::ElementRef,
    tag_stack: &mut Vec<String>,
    current_line: &mut Vec<StyledText>,
    lines: &mut Vec<Line<'static>>,
    list_depth: &mut usize,
) {
    let tag_name = element_ref.value().name().to_string();
    let is_block = is_block_element(&tag_name);
    handle_block_opening(is_block, current_line, lines);
    if tag_name == "ul" || tag_name == "ol" {
        *list_depth += 1;
    }
    if tag_name == "li" {
        add_list_prefix(current_line, *list_depth);
    }
    tag_stack.push(tag_name.clone());
    for child in element_ref.children() {
        match child.value() {
            scraper::node::Node::Element(_) => {
                if let Some(child_ref) = scraper::ElementRef::wrap(child) {
                    traverse_element(
                        child_ref,
                        tag_stack,
                        current_line,
                        lines,
                        list_depth,
                    );
                }
            }
            scraper::node::Node::Text(text) => {
                process_text_node(&text.text, tag_stack, current_line, lines);
            }
            _ => {}
        }
    }
    tag_stack.pop();
    if tag_name == "ul" || tag_name == "ol" {
        *list_depth = list_depth.saturating_sub(1);
    }
    handle_block_closing(&tag_name, is_block, current_line, lines);
    if tag_name == "br" {
        flush_current_line(current_line, lines);
    }
}

fn document_to_text(document: &str) -> Vec<Line<'static>> {
    let html = Html::parse_document(document);
    let mut lines = Vec::new();
    let mut current_line = Vec::<StyledText>::new();
    let mut list_depth = 0;
    let mut tag_stack = Vec::new();
    traverse_element(
        html.root_element(),
        &mut tag_stack,
        &mut current_line,
        &mut lines,
        &mut list_depth,
    );
    lines
        .into_iter()
        .filter(|line| {
            !line.spans.is_empty()
                && line.spans.iter().any(|span| !span.content.is_empty())
        })
        .collect()
}

fn render_selected_entry(
    frame: &mut Frame,
    area: Rect,
    entry_body: &str,
    selected_window: SelectedWindow,
    scroll_offset: u16,
) {
    let mut block = Block::new().borders(Borders::ALL);
    if selected_window == SelectedWindow::EntryContent {
        block = block.border_style(Style::new().blue());
    }
    let lines = document_to_text(entry_body);
    let paragraph = Paragraph::new(lines)
        .scroll((scroll_offset, 0))
        .block(block)
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
            state.selected_window.clone(),
            state.entry_scroll_offset,
        );
    }
}
