use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub enum Incoming {
    Login {
        username: String,
    },
    Chat {
        message: String,
    },
    Walk {
        x: i32,
        y: i32,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerState {
    pub username: String,
    pub x: i32,
    pub y: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub xp: i32,
    pub level: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Outgoing {
    Chat {
        sender: String,
        message: String,
        tick: i64,
    },
    State {
        tick: i64,
        players: Vec<PlayerState>,
    },
}
