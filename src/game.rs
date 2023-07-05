use rand::Rng;
use serde::{Deserialize, Serialize};

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
    pub fn quantity_limit(&self) -> u8 {
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
    pub piles: Vec<u8>,
    pub selected_pile: usize,
    pub current_player: CurrentPlayer,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum CurrentPlayer {
    Host,
    Guest,
}

impl CurrentPlayer {
    pub fn flip(&mut self) {
        *self = match self {
            Self::Host => Self::Guest,
            Self::Guest => Self::Host,
        }
    }
}

impl GameState {
    pub fn new(pile_amount: PileAmount, pile_sizes: PileSize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            piles: (0..pile_amount.amount())
                .map(|_| rng.gen_range(1..=pile_sizes.quantity_limit()))
                .collect(),
            selected_pile: 0,
            current_player: CurrentPlayer::Host,
        }
    }

    pub fn pick(&mut self, amount: u8) {
        self.piles[self.selected_pile] -= amount;
        self.current_player.flip();
    }

    pub fn move_selection(&mut self, delta: i8) {
        let new_selection = (self.selected_pile as i8 + delta) + self.piles.len() as i8;
        self.selected_pile = (new_selection % self.piles.len() as i8) as usize;
    }

    pub fn is_game_over(&self) -> bool {
        self.piles.iter().all(|&pile| pile == 0)
    }
}
