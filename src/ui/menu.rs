use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::utils::get_center_of_rect_for_text;

pub enum MenuState {
    MainMenu { selected: Option<bool> },
    GameSettings,
    ConnectToPeer,
    WaitingForConnection,
}

impl MenuState {
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>) {
        match self {
            MenuState::MainMenu { selected } => {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(2)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(frame.size());

                let simple_block = Block::default().borders(Borders::ALL);

                frame.render_widget(simple_block.clone(), chunks[0]);
                frame.render_widget(simple_block.clone(), chunks[1]);

                let create_game_text = Paragraph::new("Create Game");
                let connect_to_game_text = Paragraph::new("Connect to Game");

                frame.render_widget(
                    create_game_text,
                    get_center_of_rect_for_text(&chunks[0], "Create Game"),
                );
                frame.render_widget(
                    connect_to_game_text,
                    get_center_of_rect_for_text(&chunks[1], "Connect to Game"),
                );

                if let Some(selected) = selected {
                    let selected_block = Block::default()
                        .borders(Borders::ALL)
                        .border_type(tui::widgets::BorderType::Thick)
                        .border_style(Style::default().fg(tui::style::Color::Green));

                    frame.render_widget(selected_block, chunks[*selected as usize]);
                }
            }
            MenuState::GameSettings => todo!(),
            MenuState::ConnectToPeer => todo!(),
            MenuState::WaitingForConnection => todo!(),
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        match self {
            MenuState::MainMenu { selected } => match key {
                KeyCode::Left => {
                    *selected = Some(false);
                }
                KeyCode::Right => {
                    *selected = Some(true);
                }
                KeyCode::Enter => match selected {
                    Some(true) => {
                        *self = MenuState::ConnectToPeer;
                    }
                    Some(false) => {
                        *self = MenuState::GameSettings;
                    }
                    None => {
                        *selected = Some(false);
                    }
                },
                _ => {}
            },
            MenuState::GameSettings => todo!(),
            MenuState::ConnectToPeer => todo!(),
            MenuState::WaitingForConnection => todo!(),
        }
    }
}
