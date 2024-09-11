use crate::db::Database;
use crate::models::{Game, InitGameRequest};
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

mod db;
mod models;

#[get("/games")]
async fn get_all_games(db: Data<Database>) -> impl Responder {
    let games = db.get_all_games_db().await;
    match games {
        Some(games) => HttpResponse::Ok().body(format!("{:?}", games)),
        None => HttpResponse::Ok().body("No games found"),
    }
}

#[post("/games")]
async fn add_game(body: Json<InitGameRequest>, db: Data<Database>) -> impl Responder {
    let game_pubkey = body.game_pubkey.clone();
    let mut buffer = uuid::Uuid::encode_buffer();
    let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

    let new_game = Database::add_game_db(&db, Game::new(String::from(new_uuid), game_pubkey))
        .await
        .unwrap();

    HttpResponse::Ok().body(format!("{:?}", new_game))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("error connecting to database");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(add_game)
            .service(get_all_games)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
