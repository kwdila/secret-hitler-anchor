use crate::db::{
    add_game, add_player, all_games, game_by_id, game_by_pubkey, player_by_pubkey,
    players_by_game_id, Game, Player,
};
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;

pub fn games_service() -> Router {
    Router::new()
        .route("/", get(get_all_games).post(add_new_game))
        .route("/id/:id", get(get_game_by_id))
        .route("/pubkey/:pubkey", get(get_game_by_pubkey))
}

pub fn players_service() -> Router {
    Router::new()
        .route("/id/:id", get(get_players_by_game_id))
        .route("/pubkey/:pubkey", get(get_player_by_pubkey))
        .route("/", post(add_new_player))
}

async fn get_all_games(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    match all_games(&pool).await {
        Ok(games) => Json(games).into_response(),
        Err(err) => {
            eprintln!("Error fetching games: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch games").into_response()
        }
    }
}

async fn get_game_by_id(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match game_by_id(&pool, id).await {
        Ok(game) => Json(game).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn get_game_by_pubkey(
    Extension(pool): Extension<SqlitePool>,
    Path(pubkey): Path<String>,
) -> impl IntoResponse {
    match game_by_pubkey(&pool, &pubkey).await {
        Ok(game) => Json(game).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn add_new_game(
    Extension(pool): Extension<SqlitePool>,
    Json(game): Json<Game>, // Directly use game instead of mutable
) -> impl IntoResponse {
    if game.pubkey.len() != 32 {
        return (
            StatusCode::BAD_REQUEST,
            "pubkey must be exactly 32 characters long",
        )
            .into_response();
    }

    let result = add_game(&pool, &game.pubkey).await;

    match result {
        Ok(new_id) => Json(new_id).into_response(),
        Err(err) => {
            eprintln!("Error adding game: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to add game").into_response()
        }
    }
}

async fn get_players_by_game_id(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match players_by_game_id(&pool, id).await {
        Ok(players) => Json(players).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn get_player_by_pubkey(
    Extension(pool): Extension<SqlitePool>,
    Path(pubkey): Path<String>,
) -> impl IntoResponse {
    match player_by_pubkey(&pool, &pubkey).await {
        Ok(player) => Json(player).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn add_new_player(
    Extension(pool): Extension<SqlitePool>,
    Json(player): Json<Player>,
) -> impl IntoResponse {
    if player.pubkey.len() != 32 {
        return (
            StatusCode::BAD_REQUEST,
            "pubkey must be exactly 32 characters long",
        )
            .into_response();
    }
    if ![0, 1, 2].contains(&player.role) {
        return (
            StatusCode::BAD_REQUEST,
            "player role must one of 0,1, and 2",
        )
            .into_response();
    }

    let result = add_player(&pool, &player.pubkey, player.role, player.game_id).await;

    match result {
        Ok(new_id) => Json(new_id).into_response(),
        Err(err) => {
            eprintln!("Error adding player: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to add player").into_response()
        }
    }
}
