use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct StringForm {
    label: String,
    expected_input_size: u16,
    pub state: String,
}

impl StringForm {
    pub fn new(label: String, expected_input_size: u16, state: Option<String>) -> Self {
        Self {
            state: state.unwrap_or_default(),
            label,
            expected_input_size,
        }
    }

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let screen = frame.size();

        let inner = super::utils::get_center_of_rect_for_rect(
            &screen,
            (self.state.len() as u16 + 5).max(self.expected_input_size),
            7,
        );

        let popup_block = Block::default()
            .borders(Borders::ALL)
            .title(self.label.as_str())
            .title_alignment(Alignment::Center);

        frame.render_widget(popup_block, inner);

        let popup_text = Paragraph::new(self.state.as_str()).wrap(Wrap { trim: false });
        let shifted_inner = Rect {
            x: inner.x + 2,
            // center height
            y: inner.y + inner.height / 2,
            width: inner.width - 3,
            height: inner.height,
        };
        frame.render_widget(popup_text, shifted_inner);
        frame.set_cursor(shifted_inner.x + self.state.len() as u16, shifted_inner.y)
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.state.push(c);
            }
            KeyCode::Backspace => {
                self.state.pop();
            }
            _ => {}
        }
    }

    pub fn consume(&self) -> String {
        self.state.clone()
    }
}
