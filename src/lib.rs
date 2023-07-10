use std::sync::{Arc, Mutex};

use comms::{
    client::Client,
    server::{Server, ServerMessage},
};
use crossterm::event::KeyCode;
use game::GameState;
use tui::{backend::Backend, Frame};
use ui::{
    menu::{MenuState, MenuStateTransition},
    popup::Popup,
};

pub mod comms;
pub mod game;
pub mod ui;

pub struct App {
    pub state: AppState,
}

pub enum AppState {
    Menu(MenuState),
    Game(Arc<Mutex<GameState>>, Arc<Server>, Option<Client>),
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Menu(MenuState::MainMenu { selected: None }),
        }
    }

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
        match &mut self.state {
            AppState::Menu(menu_state) => menu_state.render(frame),
            AppState::Game(game, server, client) => {
                if client.is_none() {
                    Popup::new(
                        "Waiting for connection".into(),
                        format!("Opponent should connect to ip {}", server.url),
                    )
                    .render(frame);
                    return;
                }
                game.lock().unwrap().render(frame);
            }
        }
    }

    pub async fn handle_key(&mut self, key: KeyCode) {
        match &mut self.state {
            AppState::Menu(menu_state) => match menu_state.handle_key(key).await {
                MenuStateTransition::GameOpen(server, game) => {
                    self.state = AppState::Game(game, server, None);
                }
                MenuStateTransition::ConnectedToPeer(server, game, clt) => {
                    self.state = AppState::Game(game, server, Some(clt));
                }
                MenuStateTransition::Continue => {}
            },
            AppState::Game(game, _, client) => {
                if client.is_none() {
                    // waiting for connection
                    return;
                }

                let mut game_state = game.lock().unwrap();

                if game_state.handle_key(key) {
                    client
                        .as_ref()
                        .unwrap()
                        .send_game_state(&game_state)
                        .await
                        .unwrap();
                }
            }
        }
    }

    pub async fn on_tick(&mut self) {
        match &mut self.state {
            AppState::Game(_, server, client) => {
                if client.is_none() {
                    if let Some(ServerMessage::GuestConnected(url)) =
                        server.messages.lock().unwrap().pop_front()
                    {
                        let clt = Client::new(url);
                        if let Err(e) = clt.check_connection().await {
                            println!("error connecting to client: {e}");
                            return;
                        }
                        *client = Some(clt);
                    }
                }
            }
            _ => {}
        }
    }
}
