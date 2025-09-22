use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, web};
use protocol::PlayerAction;
use uuid::Uuid;

use crate::state::AppState;

/// Applies a player move to a specific game.
///
/// This endpoint accepts a player action (such as movement) and applies it to the game
/// identified by the UUID in the path. The game state is updated and the result
/// of the action is returned.
///
/// # Parameters
///
/// * `path` - The game ID extracted from the URL path
/// * `data` - Shared application state containing all active games
/// * `json` - The player action to be applied to the game
///
/// # Returns
///
/// Returns a JSON response containing the step result, including events that occurred
/// and any state changes.
pub async fn apply_move(
    path: web::Path<Uuid>,
    data: web::Data<Arc<Mutex<AppState>>>,
    json: web::Json<PlayerAction>,
) -> HttpResponse {
    let game_id = path.into_inner();

    let engine = {
        let Ok(guard) = data.lock() else {
            return HttpResponse::InternalServerError()
                .json("Failed to acquire lock");
        };
        match guard.get_game(game_id) {
            Some(engine) => engine,
            None => return HttpResponse::NotFound().json("Game not found"),
        }
    };

    let mut engine_guard = engine.lock().await;
    let Ok(result) = engine_guard.apply_step(json.into_inner()).await else {
        return HttpResponse::InternalServerError()
            .json("Failed to apply move");
    };

    HttpResponse::Ok().json(result)
}
