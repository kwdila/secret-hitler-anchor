use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Game {
    pub pubkey: String,
}
