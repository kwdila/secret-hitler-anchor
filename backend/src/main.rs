use models::Game;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{SqliteError, SqliteQueryResult},
    Connection, FromRow, Row, Sqlite, SqliteConnection, SqlitePool,
};
mod models;
mod types;

use crate::models::player::Player;
use crate::types::role::Role;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db_url = "sqlite://sqlite.db";
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        print!("Database already exists");
    }
    let db = SqlitePool::connect(db_url).await.unwrap();
    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS games (id INTEGER PRIMARY KEY AUTOINCREMENT, pubkey TEXT UNIQUE NOT NULL);",
    )
    .execute(&db)
    .await
    .unwrap();
    println!("Create games table result: {:?}", result);

    let result = sqlx::query("CREATE TABLE IF NOT EXISTS players (id INTEGER PRIMARY KEY AUTOINCREMENT, role INTEGER NOT NULL, game_id INTEGER NOT NULL, FOREIGN KEY (game_id) REFERENCES games (id));").execute(&db).await.unwrap();
    println!("Create player table result: {:?}", result);
    let result = sqlx::query(
        "SELECT name
         FROM sqlite_schema
         WHERE type ='table' 
         AND name NOT LIKE 'sqlite_%';",
    )
    .fetch_all(&db)
    .await
    .unwrap();
    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }
    let result = sqlx::query("INSERT INTO games (pubkey) VALUES (?)")
        .bind("111111")
        .execute(&db)
        .await
        .unwrap();
    println!("Query result: {:?}", result);
    let game_results = sqlx::query_as::<_, Game>("SELECT id, pubkey FROM games")
        .fetch_all(&db)
        .await
        .unwrap();
    for game in game_results {
        println!("game key: {:?}", game.pubkey);
    }
    let result = sqlx::query("INSERT INTO players (role,game_id) VALUES (?,?)")
        .bind(0)
        .bind(1)
        .execute(&db)
        .await
        .unwrap();
    println!("Query result: {:?}", result);
    let player_results = sqlx::query_as::<_, Player>("SELECT id, role FROM players")
        .fetch_all(&db)
        .await
        .unwrap();
    for player in player_results {
        println!("role: {:?}", Role::from(player.role).unwrap());
    }
    Ok(())
}
