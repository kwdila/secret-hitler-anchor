use crate::models::Game;
use actix_web::web::Data;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client
            .signin(Root {
                username: "secret",
                password: "secret",
            })
            .await?;
        client
            .use_ns("surreal")
            .use_db("secret-games")
            .await
            .unwrap();
        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("secret-games"),
        })
    }
    pub async fn add_game_db(db: &Data<Database>, new_game: Game) -> Option<Game> {
        let created_pizza = db
            .client
            .create(("game", new_game.uuid.clone()))
            .content(new_game)
            .await;

        created_pizza.unwrap_or(None)
    }

    pub async fn get_all_games_db(&self) -> Option<Vec<Game>> {
        let result = self.client.select("game").await;
        match result {
            Ok(games) => Some(games),
            Err(_) => None,
        }
    }
}
