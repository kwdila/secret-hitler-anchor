use anyhow::Result as AnyhowResult;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, QueryBuilder, Row, Sqlite, SqlitePool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Game {
    pub id: Option<i32>,
    pub pubkey: String,
    pub host_key: String,
}
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Player {
    pub id: Option<i32>,
    pub pubkey: String,
    pub role: u8,
    pub game_key: String,
    pub game_id: Option<i32>,
}

pub async fn init_db() -> AnyhowResult<SqlitePool> {
    let database_url = std::env::var("DATABASE_URL")?;
    let connection_pool = SqlitePool::connect(&database_url).await?;
    sqlx::migrate!().run(&connection_pool).await?;
    Ok(connection_pool)
}
pub async fn all_games(connection_pool: &SqlitePool) -> AnyhowResult<Vec<Game>> {
    let games = sqlx::query_as::<_, Game>("SELECT * FROM games ORDER BY id")
        .fetch_all(connection_pool)
        .await?;

    Ok(games)
}
pub async fn game_by_id(connection_pool: &SqlitePool, id: i32) -> AnyhowResult<Game> {
    Ok(sqlx::query_as::<_, Game>("SELECT * FROM games WHERE id=$1")
        .bind(id)
        .fetch_one(connection_pool)
        .await?)
}
pub async fn game_id_by_pubkey(connection_pool: &SqlitePool, pubkey: &str) -> AnyhowResult<i32> {
    Ok(sqlx::query("SELECT id FROM games WHERE pubkey=$1")
        .bind(pubkey)
        .fetch_one(connection_pool)
        .await?
        .get(0))
}
pub async fn add_game<S: ToString>(
    connection_pool: &SqlitePool,
    pubkey: S,
    host_key: S,
) -> AnyhowResult<i32> {
    let pubkey = pubkey.to_string();
    let host_key = host_key.to_string();
    let id = sqlx::query("INSERT INTO games (pubkey,host_key) VALUES ($1,$2) RETURNING id")
        .bind(pubkey)
        .bind(host_key)
        .fetch_one(connection_pool)
        .await?
        .get(0);

    Ok(id)
}
pub async fn player_by_pubkey(connection_pool: &SqlitePool, pubkey: &str) -> AnyhowResult<Player> {
    Ok(
        sqlx::query_as::<_, Player>("SELECT * FROM players WHERE pubkey=$1")
            .bind(pubkey)
            .fetch_one(connection_pool)
            .await?,
    )
}
pub async fn players_by_game_id(
    connection_pool: &SqlitePool,
    game_id: i32,
) -> AnyhowResult<Vec<Player>> {
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
) -> AnyhowResult<i32> {
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
pub async fn add_many_players(
    connection_pool: &SqlitePool,
    players: Vec<Player>,
) -> AnyhowResult<u64> {
    let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO players (pubkey, role,game_id,game_key) ");

    query_builder.push_values(players.into_iter(), |mut b, player| {
        b.push_bind(player.pubkey)
            .push_bind(player.role)
            .push_bind(player.game_id)
            .push_bind(player.game_key);
    });

    let query = query_builder.build();

    let result = query.execute(connection_pool).await?;
    Ok(result.rows_affected())
}
