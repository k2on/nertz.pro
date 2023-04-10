use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub players: Vec<Player>,
    pub first_to: u32,
    pub rounds: Vec<Round>,
    pub is_game_started: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            players: Vec::new(),
            first_to: 100,
            rounds: Vec::new(),
            is_game_started: false,
        }
    }

    pub fn player_remove(&mut self, idx: usize) {
        self.players.remove(idx);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
}

pub type PlayerId = u32;

#[derive(Debug, Serialize, Deserialize)]
pub struct Round {
    pub scores: Vec<Score>,
}

pub type Score = i8;

impl State {}
