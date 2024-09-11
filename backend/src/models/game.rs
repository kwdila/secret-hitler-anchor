use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct InitGameRequest {
    pub game_pubkey: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub uuid: String,
    pub pubkey: String,
}

impl Game {
    pub fn new(uuid: String, pubkey: String) -> Game {
        Game { uuid, pubkey }
    }
}
