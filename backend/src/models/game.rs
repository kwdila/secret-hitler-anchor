#[derive()]
pub struct InitGameRequest {
    pub game_pubkey: String,
}
#[derive(Debug)]
pub struct Game {
    pub uuid: String,
    pub pubkey: String,
}

impl Game {
    pub fn new(uuid: String, pubkey: String) -> Game {
        Game { uuid, pubkey }
    }
}
