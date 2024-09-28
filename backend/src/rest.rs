use crate::db::{
    add_game, add_player, all_games, game_by_id, game_by_pubkey, player_by_pubkey,
    players_by_game_id, Game, Player,
};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{extract, Extension, Json, Router};
use solana_sdk::pubkey;
use sqlx::SqlitePool;

pub fn games_service() -> Router {
    Router::new()
        .route("/", get(all_games))
        .route("/:id", get(game_by_id))
        .route("/:pubkey", get(game_by_pubkey))
        .route("/add", post(add_game))
}
pub fn players_service() -> Router {
    Router::new()
        .route("/:id", get(players_by_game_id))
        .route("/:pubkey", get(player_by_pubkey))
        .route("/add", post(add_player))
}

async fn get_all_games(
    Extension(cnn): Extension<SqlitePool>,
) -> Result<Json<Vec<Game>>, StatusCode> {
    if let Ok(games) = all_games(&cnn).await {
        Ok(Json(games))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn get_game_by_id(
    Extension(cnn): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<Game>, StatusCode> {
    if let Ok(game) = game_by_id(&cnn, id).await {
        Ok(Json(game))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}
async fn get_game_by_pubkey(
    Extension(cnn): Extension<SqlitePool>,
    Path(pubkey): Path<&str>,
) -> Result<Json<Game>, StatusCode> {
    if let Ok(game) = game_by_pubkey(&cnn, pubkey).await {
        Ok(Json(game))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn add_new_game(
    Extension(cnn): Extension<SqlitePool>,
    extract::Json(game): extract::Json<Game>,
) -> Result<Json<i32>, StatusCode> {
    if let Ok(new_id) = crate::db::add_game(&cnn, &game.pubkey).await {
        Ok(Json(new_id))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn get_players_by_game_id(
    Extension(cnn): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<Vec<Player>>, StatusCode> {
    if let Ok(players) = players_by_game_id(&cnn, id).await {
        Ok(Json(players))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}
async fn get_player_by_pubkey(
    Extension(cnn): Extension<SqlitePool>,
    Path(pubkey): Path<&str>,
) -> Result<Json<Player>, StatusCode> {
    if let Ok(player) = player_by_pubkey(&cnn, pubkey).await {
        Ok(Json(player))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn add_new_player(
    Extension(cnn): Extension<SqlitePool>,
    extract::Json(player): extract::Json<Player>,
) -> Result<Json<i32>, StatusCode> {
    if let Ok(new_id) =
        crate::db::add_player(&cnn, &player.pubkey, player.role, player.game_id).await
    {
        Ok(Json(new_id))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}
