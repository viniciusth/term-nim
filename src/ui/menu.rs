use std::{
    net::{IpAddr, Ipv4Addr},
    sync::{Arc, Mutex},
};

use crossterm::event::KeyCode;
use local_ip_address::local_ip;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Margin},
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    comms::{client::Client, server::Server},
    game::{GameState, PileAmount, PileSize, PlayerType},
};

use super::{form::StringForm, stateful_list::StatefulList, utils::get_center_of_rect_for_text};

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
        client: Option<Client>,
    },
    WaitingForConnection {
        form: StringForm,
        game: GameState,
    },
}

pub enum MenuStateTransition {
    Continue,
    GameOpen(Arc<Server>, Arc<Mutex<GameState>>),
    ConnectedToPeer(Arc<Server>, Arc<Mutex<GameState>>, Client),
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
                        .border_type(tui::widgets::BorderType::Double)
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
                        .border_type(tui::widgets::BorderType::Double)
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
            MenuState::ConnectToPeer { form, .. } => {
                form.render(frame);
            }
            MenuState::WaitingForConnection { form, .. } => {
                form.render(frame);
            }
        }
    }

    pub async fn handle_key(&mut self, key: KeyCode) -> MenuStateTransition {
        match self {
            MenuState::MainMenu { selected } => {
                match key {
                    KeyCode::Left => {
                        *selected = Some(false);
                    }
                    KeyCode::Right => {
                        *selected = Some(true);
                    }
                    KeyCode::Enter => match selected {
                        Some(true) => {
                            *self = MenuState::ConnectToPeer {
                                form: StringForm::new(
                                    "Connect to peer".into(),
                                    20,
                                    Some(
                                        local_ip()
                                            .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST))
                                            .to_string()
                                            + ":",
                                    ),
                                ),
                                client: None,
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
                }

                MenuStateTransition::Continue
            }
            MenuState::GameSettings {
                selected,
                amounts,
                sizes,
            } => {
                match key {
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
                            let default_addr = format!(
                                "{}:4088",
                                local_ip().unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST))
                            );
                            *self = MenuState::WaitingForConnection {
                                form: StringForm::new(
                                    "IP to expose".into(),
                                    default_addr.len() as u16,
                                    Some(default_addr),
                                ),
                                game: GameState::new(
                                    amounts.get_selected().unwrap(),
                                    sizes.get_selected().unwrap(),
                                    PlayerType::Host,
                                ),
                            };
                        }
                        None => {
                            *selected = Some(false);
                        }
                    },
                    _ => {}
                }

                MenuStateTransition::Continue
            }
            MenuState::ConnectToPeer { form, client } => {
                if key == KeyCode::Enter {
                    if let Some(clt) = client {
                        let addr = form.consume();
                        let server = Arc::new(Server::new(addr.clone(), Default::default()));
                        let mut initial_state = clt.connect_to_game(addr).await.unwrap();
                        initial_state.player_type = PlayerType::Guest;
                        *server.current_game_state.lock().unwrap() = initial_state;
                        let state = server.current_game_state.clone();
                        tokio::spawn(server.clone().start());
                        MenuStateTransition::ConnectedToPeer(server, state, clt.clone())
                    } else {
                        let addr = form.consume();
                        let clt = Client::new(addr);
                        if let Err(e) = clt.check_connection().await {
                            form.state = format!("Failed to connect: {e:?}");
                            return MenuStateTransition::Continue;
                        }
                        *client = Some(clt);
                        let default_addr = format!(
                            "{}:4088",
                            local_ip().unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST))
                        );
                        *form = StringForm::new(
                            "IP to expose".into(),
                            default_addr.len() as u16,
                            Some(default_addr),
                        );
                        MenuStateTransition::Continue
                    }
                } else {
                    form.handle_key(key);
                    MenuStateTransition::Continue
                }
            }
            MenuState::WaitingForConnection { form, game } => {
                if key == KeyCode::Enter {
                    let addr = form.consume();
                    let game = Arc::new(Mutex::new(game.clone()));
                    let server = Arc::new(Server::new(addr, game.clone()));
                    tokio::spawn(server.clone().start());
                    MenuStateTransition::GameOpen(server, game)
                } else {
                    form.handle_key(key);
                    MenuStateTransition::Continue
                }
            }
        }
    }
}
