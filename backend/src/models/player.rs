use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct Player {
    pub role: u8,
}
