use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/games")]
async fn get_all_games() -> impl Responder {
    HttpResponse::Ok().body("meow all games")
}

#[post("/games")]
async fn add_game() -> impl Responder {
    HttpResponse::Ok().body("game initialized")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_all_games).service(add_game))
        .bind("localhost:8080")?
        .run()
        .await
}
