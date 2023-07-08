use tui::{
    backend::Backend,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::utils::{get_center_of_rect_for_rect, get_center_of_rect_for_text};

pub struct Popup {
    title: String,
    body: String,
}

impl Popup {
    pub fn new(title: String, body: String) -> Self {
        Self { title, body }
    }

    pub fn render<B: Backend>(&self, frame: &mut Frame<B>) {
        let screen = frame.size();

        let inner = get_center_of_rect_for_rect(&screen, self.body.len() as u16 + 20, 10);

        let popup_block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.as_str())
            .title_alignment(Alignment::Center);

        frame.render_widget(popup_block, inner);

        let popup_text = Paragraph::new(self.body.as_str());

        frame.render_widget(popup_text, get_center_of_rect_for_text(&inner, &self.body));
    }
}
