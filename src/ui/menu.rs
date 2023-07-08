use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Margin},
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::game::{PileAmount, PileSize};

use super::{
    form::StringForm, popup::Popup, stateful_list::StatefulList, utils::get_center_of_rect_for_text,
};

pub enum MenuState {
    MainMenu {
        selected: Option<bool>,
    },
    GameSettings {
        selected: Option<bool>,
        amounts: StatefulList<PileAmount>,
        sizes: StatefulList<PileSize>,
    },
    ConnectToPeer {
        form: StringForm,
    },
    WaitingForConnection {
        popup: Popup,
    },
}

impl MenuState {
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
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
            MenuState::GameSettings {
                selected,
                amounts,
                sizes,
            } => {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(2)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(frame.size());

                let simple_block = Block::default().borders(Borders::ALL);

                let titles = ["Pile Amount", "Pile Size"];

                frame.render_widget(simple_block.clone().title(titles[0]), chunks[0]);
                frame.render_widget(simple_block.clone().title(titles[1]), chunks[1]);

                if let Some(selected) = selected {
                    let selected_block = Block::default()
                        .borders(Borders::ALL)
                        .border_type(tui::widgets::BorderType::Thick)
                        .border_style(Style::default().fg(tui::style::Color::Green))
                        .title(titles[*selected as usize]);

                    frame.render_widget(selected_block, chunks[*selected as usize]);
                }

                amounts.render(
                    frame,
                    chunks[0].inner(&Margin {
                        vertical: 1,
                        horizontal: 1,
                    }),
                );

                sizes.render(
                    frame,
                    chunks[1].inner(&Margin {
                        vertical: 1,
                        horizontal: 1,
                    }),
                );
            }
            MenuState::ConnectToPeer { form } => {
                form.render(frame);
            }
            MenuState::WaitingForConnection { popup } => popup.render(frame),
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
                        *self = MenuState::ConnectToPeer {
                            form: StringForm::new("Connect to peer".into(), 20),
                        };
                    }
                    Some(false) => {
                        *self = MenuState::GameSettings {
                            selected: None,
                            amounts: StatefulList::with_items(vec![
                                PileAmount::Two,
                                PileAmount::Five,
                                PileAmount::Ten,
                            ]),
                            sizes: StatefulList::with_items(vec![
                                PileSize::Small,
                                PileSize::Medium,
                                PileSize::Large,
                            ]),
                        };
                    }
                    None => {
                        *selected = Some(false);
                    }
                },
                _ => {}
            },
            MenuState::GameSettings {
                selected,
                amounts,
                sizes,
            } => match key {
                KeyCode::Left => {
                    *selected = Some(false);
                }
                KeyCode::Right => {
                    *selected = Some(true);
                }
                KeyCode::Up => match selected {
                    Some(true) => {
                        sizes.previous();
                    }
                    Some(false) => {
                        amounts.previous();
                    }
                    _ => {}
                },
                KeyCode::Down => match selected {
                    Some(true) => {
                        sizes.next();
                    }
                    Some(false) => {
                        amounts.next();
                    }
                    _ => {}
                },
                KeyCode::Enter => match selected {
                    Some(_) => {
                        *self = MenuState::WaitingForConnection {
                            popup: Popup::new(
                                "Waiting for connection".into(),
                                "Should connect to ip lalala:3020".into(),
                            ),
                        };
                    }
                    None => {
                        *selected = Some(false);
                    }
                },
                _ => {}
            },
            MenuState::ConnectToPeer { form } => form.handle_key(key),
            MenuState::WaitingForConnection { .. } => {}
        }
    }
}
