use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

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
pub async fn all_games(connection_pool: &SqlitePool) -> Result<Vec<Game>> {
    if let Some(all_games) = CACHE.all_game().await {
        Ok(all_games)
    } else {
        let games = sqlx::query_as::<_, Game>("SELECT * FROM games")
            .fetch_all(connection_pool)
            .await?;
        CACHE.refresh(games.clone()).await;
        Ok(games)
    }
}
