use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub enum Incoming {
    Login {
        username: String,
    },
    Chat {
        message: String,
    },
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Outgoing {
    Chat {
        sender: String,
        message: String,
        tick: i64,
    },
}
