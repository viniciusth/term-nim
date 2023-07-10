use crossterm::event::KeyCode;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::ui::{popup::Popup, utils::get_center_of_rect_for_text};

pub enum PileAmount {
    Two,
    Five,
    Ten,
}

impl PileAmount {
    pub fn amount(&self) -> usize {
        match self {
            Self::Two => 2,
            Self::Five => 5,
            Self::Ten => 10,
        }
    }
}

impl ToString for PileAmount {
    fn to_string(&self) -> String {
        match self {
            Self::Two => "Two",
            Self::Five => "Five",
            Self::Ten => "Ten",
        }
        .to_string()
    }
}

pub enum PileSize {
    Small,
    Medium,
    Large,
}

impl PileSize {
    pub fn quantity_limit(&self) -> i8 {
        match self {
            Self::Small => 5,
            Self::Medium => 10,
            Self::Large => 20,
        }
    }
}

impl ToString for PileSize {
    fn to_string(&self) -> String {
        match self {
            Self::Small => "Small",
            Self::Medium => "Medium",
            Self::Large => "Large",
        }
        .to_string()
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GameState {
    pub piles: Vec<i8>,
    pub selected_pile: usize,
    pub amount_selected: Option<i8>,
    pub current_player: PlayerType,
    pub player_type: PlayerType,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            piles: vec![],
            selected_pile: 0,
            amount_selected: None,
            current_player: PlayerType::Host,
            player_type: PlayerType::Host,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum PlayerType {
    Host,
    Guest,
}

impl PlayerType {
    pub fn flip(&mut self) {
        *self = match self {
            Self::Host => Self::Guest,
            Self::Guest => Self::Host,
        }
    }
}

impl GameState {
    pub fn new(pile_amount: &PileAmount, pile_sizes: &PileSize, player_type: PlayerType) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            piles: (0..pile_amount.amount())
                .map(|_| rng.gen_range(1..=pile_sizes.quantity_limit()))
                .collect(),
            selected_pile: 0,
            current_player: player_type.clone(),
            player_type,
            amount_selected: None,
        }
    }

    pub fn pick(&mut self) {
        self.piles[self.selected_pile] -= self.amount_selected.take().unwrap();
        self.current_player.flip();
    }

    pub fn next(&mut self) {
        self.selected_pile = (self.selected_pile + 1) % self.piles.len();
    }

    pub fn previous(&mut self) {
        self.selected_pile = (self.selected_pile + self.piles.len() - 1) % self.piles.len();
    }

    pub fn is_game_over(&self) -> bool {
        self.piles.iter().all(|&pile| pile == 0)
    }

    /// Handles keyboard input from the user, returns true if the game state was changed.
    pub fn handle_key(&mut self, key: KeyCode) -> bool {
        if self.current_player != self.player_type {
            return false;
        }
        match key {
            KeyCode::Left => {
                self.amount_selected = None;
                self.previous();
            }
            KeyCode::Right => {
                self.amount_selected = None;
                self.next();
            }
            KeyCode::Up => {
                if let Some(amount) = self.amount_selected {
                    self.amount_selected = Some((amount - 1).max(0));
                } else {
                    self.amount_selected = Some(0);
                }
            }
            KeyCode::Down => {
                if let Some(amount) = self.amount_selected {
                    self.amount_selected = Some((amount + 1).min(self.piles[self.selected_pile]));
                } else {
                    self.amount_selected = Some(1);
                }
            }
            KeyCode::Enter => {
                if let Some(amount) = self.amount_selected {
                    if amount > 0 {
                        self.pick();
                    }
                }
            }
            _ => return false,
        }
        true
    }

    pub fn render<B: Backend>(&self, frame: &mut Frame<B>) {
        if self.is_game_over() {
            let popup = if self.player_type != self.current_player {
                Popup::new("Game Over".into(), "You Won! :)".into())
            } else {
                Popup::new("Game Over".into(), "You Lose! :(".into())
            };

            popup.render(frame);
            return;
        }

        let screen = frame.size();

        let chunks = match self.piles.len() {
            2 => Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(screen),
            5 => Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ])
                .split(screen),
            10 => {
                let rows = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(screen);
                rows.into_iter()
                    .flat_map(|r| {
                        Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([
                                Constraint::Percentage(20),
                                Constraint::Percentage(20),
                                Constraint::Percentage(20),
                                Constraint::Percentage(20),
                                Constraint::Percentage(20),
                            ])
                            .split(r)
                    })
                    .collect()
            }
            _ => panic!("Unsupported number of piles"),
        };

        for (i, pile_qty) in self.piles.iter().enumerate() {
            let pile_block = if i == self.selected_pile {
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(
                        Style::default().fg(if self.player_type == self.current_player {
                            Color::Green
                        } else {
                            Color::Red
                        }),
                    )
                    .title(format!("Pile {}", i + 1))
                    .title_alignment(Alignment::Center)
            } else {
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Pile {}", i + 1))
                    .title_alignment(Alignment::Center)
            };

            frame.render_widget(pile_block, chunks[i]);

            let (rect, msg) = if i == self.selected_pile {
                let msg = format!(
                    "{pile_qty} => {}",
                    pile_qty - self.amount_selected.unwrap_or(0)
                );
                (get_center_of_rect_for_text(&chunks[i], &msg), msg)
            } else {
                let msg = pile_qty.to_string();
                (get_center_of_rect_for_text(&chunks[i], &msg), msg)
            };

            frame.render_widget(Paragraph::new(msg), rect);
        }
    }
}
