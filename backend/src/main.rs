use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{SqliteError, SqliteQueryResult},
    Connection, FromRow, Row, Sqlite, SqliteConnection, SqlitePool,
};
#[derive(Clone, FromRow, Debug)]
struct User {
    id: i64,
    name: String,
}

mod models;

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
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL);").execute(&db).await.unwrap();
    println!("Create user table result: {:?}", result);
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
    let result = sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind("bobby")
        .execute(&db)
        .await
        .unwrap();
    println!("Query result: {:?}", result);
    let user_results = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&db)
        .await
        .unwrap();
    for user in user_results {
        println!("[{}] name: {}", user.id, &user.name);
    }
    Ok(())
}
