use ratatui::widgets::ListState;

pub struct State {
    pub selected: usize,
    pub list_state: ListState,
    pub feeds: Vec<String>,
}

impl State {
    pub fn new(feeds: Vec<&str>) -> Self {
        let selected = 0;
        let mut list_state = ListState::default();
        list_state.select(Some(selected));
        State {
            selected,
            list_state,
            feeds: feeds.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn move_down(&mut self) {
        if self.selected < self.feeds.len().saturating_sub(1) {
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
}
