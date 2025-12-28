use feed_rs::model::{Entry, Feed};
use ratatui::widgets::ListState;

pub struct State {
    pub selected: usize,
    pub list_state: ListState,
    pub feeds: Vec<Feed>,
    pub entries: Vec<EntryWithAuthor>,
    pub render_raw_html: bool,
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
        let selected = 0;
        let mut list_state = ListState::default();
        list_state.select(Some(selected));
        let entries = entries_from_feeds(&feeds);
        State {
            selected,
            list_state,
            feeds,
            entries,
            render_raw_html: false,
        }
    }

    pub fn move_down(&mut self) {
        if self.selected < self.entries.len().saturating_sub(1) {
            self.selected += 1;
            self.list_state.select(Some(self.selected));
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            self.list_state.select(Some(self.selected));
        }
    }

    pub fn update_feeds(&mut self, feeds: Vec<Feed>) {
        self.feeds = feeds;
        self.entries = entries_from_feeds(&self.feeds);
        if self.selected >= self.entries.len() {
            self.selected = self.entries.len().saturating_sub(1);
        }
        self.list_state.select(Some(self.selected));
    }

    pub fn get_selected_entry_body(&self) -> &str {
        self.entries[self.selected]
            .entry
            .content
            .as_ref()
            .and_then(|c| c.body.as_deref())
            .unwrap_or("No Content")
    }

    pub fn switch_render_mode(&mut self) {
        self.render_raw_html = !self.render_raw_html;
    }
}

impl std::ops::Deref for State {
    type Target = Vec<EntryWithAuthor>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}
