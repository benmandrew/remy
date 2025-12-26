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
    entries.sort_by(|a, b| b.published.cmp(&a.published));
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
        // Reset selection if out of bounds
        if self.selected >= self.feeds.len() {
            self.selected = self.feeds.len().saturating_sub(1);
        }
        self.list_state.select(Some(self.selected));
    }
}

impl std::ops::Deref for State {
    type Target = Vec<Entry>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}
