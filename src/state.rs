use feed_rs::model::{Entry, Feed};
use ratatui::widgets::ListState;

pub struct State {
    pub selected: usize,
    pub list_state: ListState,
    pub feeds: Vec<Feed>,
    pub entries: Vec<Entry>,
}

fn entries_from_feeds(feeds: &Vec<Feed>) -> Vec<Entry> {
    let mut entries = vec![];
    for feed in feeds {
        for entry in &feed.entries {
            entries.push(entry.clone());
        }
    }
    entries.sort_by(|a, b| b.updated.cmp(&a.updated));
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

    pub fn get_entry_titles(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|e| {
                e.title
                    .as_ref()
                    .map(|t| t.content.clone())
                    .unwrap_or_else(|| "No Title".to_string())
            })
            .collect()
    }

    pub fn get_selected_entry_body(&self) -> &str {
        self.entries[self.selected]
            .content
            .as_ref()
            .and_then(|c| c.body.as_deref())
            .unwrap_or("No Content")
    }
}

impl std::ops::Deref for State {
    type Target = Vec<Entry>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}
