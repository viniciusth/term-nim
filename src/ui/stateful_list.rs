use tui::{
    backend::Backend,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{List, ListItem, ListState},
    Frame,
};

pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T: ToString> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut state = ListState::default();
        state.select(Some(0));
        StatefulList { state, items }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|i| ListItem::new(i.to_string()))
            .collect();
        let list = List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");
        frame.render_stateful_widget(list, area, &mut self.state);
    }
}
