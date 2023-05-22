use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub enum Packet {
    Chat {
        message: String,
    },
}
