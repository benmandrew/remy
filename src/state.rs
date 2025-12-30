use feed_rs::model::{Entry, Feed};
use ratatui::widgets::ListState;

#[derive(Clone, PartialEq)]
pub enum SelectedWindow {
    EntryList,
    EntryContent,
    HelpPopup,
}

pub struct State {
    pub selected_entry: usize,
    pub list_state: ListState,
    pub feeds: Vec<Feed>,
    pub entries: Vec<EntryWithAuthor>,
    pub render_raw_html: bool,
    pub selected_window: SelectedWindow,
    pub entry_scroll_offset: u16,
}

pub struct EntryWithAuthor {
    pub entry: Entry,
    pub author: String,
}

impl EntryWithAuthor {
    pub fn new(entry: Entry, author: Option<String>) -> Self {
        let author = if let Some(author) = entry.authors.first() {
            author.name.clone()
        } else if let Some(contributor) = entry.contributors.first() {
            contributor.name.clone()
        } else if let Some(author) = author {
            author
        } else {
            "Unknown Author".to_string()
        };
        EntryWithAuthor { entry, author }
    }
}

fn entries_from_feeds(feeds: &Vec<Feed>) -> Vec<EntryWithAuthor> {
    let mut entries = vec![];
    for feed in feeds {
        for entry in &feed.entries {
            entries.push(EntryWithAuthor::new(
                entry.clone(),
                feed.authors.first().map(|a| a.name.clone()),
            ));
        }
    }
    entries.sort_by(|a, b| b.entry.updated.cmp(&a.entry.updated));
    entries
}

impl State {
    pub fn new(feeds: Vec<Feed>) -> Self {
        let selected_entry = 0;
        let mut list_state = ListState::default();
        list_state.select(Some(selected_entry));
        let entries = entries_from_feeds(&feeds);
        State {
            selected_entry,
            list_state,
            feeds,
            entries,
            render_raw_html: false,
            selected_window: SelectedWindow::EntryList,
            entry_scroll_offset: 0,
        }
    }

    pub fn move_down(&mut self) {
        match self.selected_window {
            SelectedWindow::EntryList => {
                if self.selected_entry < self.entries.len().saturating_sub(1) {
                    self.selected_entry += 1;
                    self.list_state.select(Some(self.selected_entry));
                    self.entry_scroll_offset = 0;
                }
            }
            SelectedWindow::EntryContent => {
                self.entry_scroll_offset += 1;
            }
            SelectedWindow::HelpPopup => {}
        }
    }

    pub fn move_up(&mut self) {
        match self.selected_window {
            SelectedWindow::EntryList => {
                if self.selected_entry > 0 {
                    self.selected_entry -= 1;
                    self.list_state.select(Some(self.selected_entry));
                    self.entry_scroll_offset = 0;
                }
            }
            SelectedWindow::EntryContent => {
                self.entry_scroll_offset =
                    self.entry_scroll_offset.saturating_sub(1);
            }
            SelectedWindow::HelpPopup => {}
        }
    }

    pub fn move_left(&mut self) {
        match self.selected_window {
            SelectedWindow::HelpPopup => {}
            SelectedWindow::EntryContent | SelectedWindow::EntryList => {
                self.selected_window = SelectedWindow::EntryList;
            }
        }
    }

    pub fn move_right(&mut self) {
        match self.selected_window {
            SelectedWindow::HelpPopup => {}
            SelectedWindow::EntryContent | SelectedWindow::EntryList => {
                self.selected_window = SelectedWindow::EntryContent;
            }
        }
    }

    pub fn update_feeds(&mut self, feeds: Vec<Feed>) {
        self.feeds = feeds;
        self.entries = entries_from_feeds(&self.feeds);
        if self.selected_entry >= self.entries.len() {
            self.selected_entry = self.entries.len().saturating_sub(1);
        }
        self.list_state.select(Some(self.selected_entry));
    }

    pub fn get_selected_entry_body(&self) -> &str {
        self.entries[self.selected_entry]
            .entry
            .content
            .as_ref()
            .and_then(|c| c.body.as_deref())
            .unwrap_or("No Content")
    }

    pub fn switch_render_mode(&mut self) {
        self.render_raw_html = !self.render_raw_html;
    }

    pub fn open_selected_entry_link(&self) {
        if let Some(link) = self.entries[self.selected_entry]
            .entry
            .links
            .first()
            .map(|l| l.href.clone())
            && let Err(e) = open::that_detached(link)
        {
            eprintln!("Failed to open link: {}", e);
        }
    }
}

impl std::ops::Deref for State {
    type Target = Vec<EntryWithAuthor>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}
