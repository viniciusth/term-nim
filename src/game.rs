use rand::Rng;

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

pub struct GameState {
    pub piles: Vec<u8>,
    pub selected_pile: usize,
    pub is_current_player: bool,
}

impl GameState {
    pub fn new(pile_amount: PileAmount, pile_sizes: PileSize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            piles: (0..pile_amount.amount())
                .map(|_| rng.gen_range(1..=pile_sizes.quantity_limit()))
                .collect(),
            selected_pile: 0,
            is_current_player: true,
        }
    }

    pub fn pick(&mut self, amount: u8) {
        self.piles[self.selected_pile] -= amount;
        self.is_current_player = !self.is_current_player;
    }

    pub fn move_selection(&mut self, delta: i8) {
        let new_selection = (self.selected_pile as i8 + delta) + self.piles.len() as i8;
        self.selected_pile = (new_selection % self.piles.len() as i8) as usize;
    }

    pub fn is_game_over(&self) -> bool {
        self.piles.iter().all(|&pile| pile == 0)
    }
}
