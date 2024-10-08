use crate::db::{
    add_game, add_many_players, add_player, all_games, game_by_id, game_id_by_pubkey,
    player_by_pubkey, players_by_game_id, Game, Player,
};
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use bincode::serialize;
use serde_json::{json, Value};
use sqlx::SqlitePool;

use crate::solana::{get_start_game_ix, set_player_roles};

pub fn games_service() -> Router {
    Router::new()
        .route("/", get(get_all_games))
        .route("/id/:id", get(get_game_by_id))
        .route("/pubkey/:pubkey", get(get_game_by_pubkey))
        .route("/add", post(add_new_game))
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
    match game_id_by_pubkey(&pool, &pubkey).await {
        Ok(game) => Json(game).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn add_new_game(
    Extension(pool): Extension<SqlitePool>,
    Json(game): Json<Game>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    // Add the game to the database
    add_game(&pool, &game.pubkey, &game.host_key)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to add game: {}", err)})),
            )
        })?;

    let client =
        solana_client::nonblocking::rpc_client::RpcClient::new("http://localhost:8899".to_string());

    let players = set_player_roles(&game.pubkey.to_string().trim(), &client)
        .await
        .unwrap();

    add_many_players(&pool, players).await.unwrap();

    // Get the start game transaction
    let tx = get_start_game_ix(&game.host_key, &client)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to create start game transaction: {}", err)})),
            )
        })?;

    // Serialize the transaction
    let serialized_transaction = serialize(&tx).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to serialize transaction"})),
        )
    })?;

    // Encode the serialized transaction
    let encoded_transaction = STANDARD.encode(serialized_transaction);

    // Return the successful response
    Ok(Json(json!({
        "transaction": encoded_transaction,
        "message": "game created"
    })))
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

    let game_id = game_id_by_pubkey(&pool, &player.game_key).await.unwrap();

    let result = add_player(&pool, &player.pubkey, player.role, game_id).await;

    match result {
        Ok(new_id) => Json(new_id).into_response(),
        Err(err) => {
            eprintln!("Error adding player: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to add player").into_response()
        }
    }
}
