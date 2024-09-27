use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, SqlitePool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub pubkey: String,
}
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub pubkey: String,
    pub role: u8,
}

pub async fn init_db() -> Result<SqlitePool> {
    let database_url = std::env::var("DATABASE_URL")?;
    let connection_pool = SqlitePool::connect(&database_url).await?;
    sqlx::migrate!().run(&connection_pool).await?;
    Ok(connection_pool)
}
pub async fn game_by_id(connection_pool: &SqlitePool, id: i32) -> Result<Game> {
    Ok(sqlx::query_as::<_, Game>("SELECT * FROM books WHERE id=$1")
        .bind(id)
        .fetch_one(connection_pool)
        .await?)
}
pub async fn game_by_pubkey(connection_pool: &SqlitePool, pubkey: &str) -> Result<Game> {
    Ok(
        sqlx::query_as::<_, Game>("SELECT * FROM books WHERE pubkey=$1")
            .bind(pubkey)
            .fetch_one(connection_pool)
            .await?,
    )
}
pub async fn add_game<S: ToString>(connection_pool: &SqlitePool, pubkey: S) -> Result<i32> {
    let pubkey = pubkey.to_string();
    let id = sqlx::query("INSERT INTO books (pubkey) VALUES ($1) RETURNING id")
        .bind(pubkey)
        .fetch_one(connection_pool)
        .await?
        .get(0);
    Ok(id)
}
pub async fn player_by_pubkey(connection_pool: &SqlitePool, pubkey: &str) -> Result<Player> {
    Ok(
        sqlx::query_as::<_, Player>("SELECT * FROM players WHERE pubkey=$1")
            .bind(pubkey)
            .fetch_one(connection_pool)
            .await?,
    )
}
pub async fn players_by_game_id(connection_pool: &SqlitePool, game_id: i32) -> Result<Vec<Player>> {
    let players = sqlx::query_as::<_, Player>("SELECT * FROM players WHERE game_id=$1")
        .bind(game_id)
        .fetch_all(connection_pool)
        .await?;
    Ok(players)
}
pub async fn add_player<S: ToString>(
    connection_pool: &SqlitePool,
    pubkey: S,
    role: u8,
    game_id: i32,
) -> Result<i32> {
    let pubkey = pubkey.to_string();
    let id =
        sqlx::query("INSERT INTO players (pubkey, role,game_id) VALUES ($1, $2, $3) RETURNING id")
            .bind(pubkey)
            .bind(role)
            .bind(game_id)
            .fetch_one(connection_pool)
            .await?
            .get(0);
    Ok(id)
}
