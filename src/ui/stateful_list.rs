use tui::{
    backend::Backend,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{List, ListItem, ListState},
    Frame,
};

use super::utils::get_center_of_rect_for_list;

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

    pub fn get_selected(&self) -> Option<&T> {
        match self.state.selected() {
            Some(i) => self.items.get(i),
            None => None,
        }
    }

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let items: Vec<String> = self.items.iter().map(|i| i.to_string()).collect();
        let area = get_center_of_rect_for_list(&area, &items);
        let list = List::new(items.into_iter().map(ListItem::new).collect::<Vec<_>>())
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");
        frame.render_stateful_widget(list, area, &mut self.state);
    }
}
