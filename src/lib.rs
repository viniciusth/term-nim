use crossterm::event::KeyCode;
use game::{GameState, PileAmount, PileSize};
use tui::{backend::Backend, Frame};
use ui::menu::MenuState;

pub mod comms;
pub mod game;
pub mod ui;

pub struct App {
    pub state: AppState,
}

pub enum AppState {
    Menu(MenuState),
    Game(GameState),
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
            AppState::Game(game_state) => todo!(),
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        match &mut self.state {
            AppState::Menu(menu_state) => menu_state.handle_key(key),
            AppState::Game(ref game_state) => todo!(),
        }
    }
}
