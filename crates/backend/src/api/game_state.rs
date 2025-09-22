use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::state::AppState;

/// Retrieves the current state of a specific game.
///
/// This endpoint returns the current state of the game identified by the UUID
/// in the path. The state includes information about the player position and
/// other game elements.
///
/// # Parameters
///
/// * `path` - The game ID extracted from the URL path
/// * `data` - Shared application state containing all active games
///
/// # Returns
///
/// Returns a JSON response containing the current game state.
pub async fn game_state(
    path: web::Path<Uuid>,
    data: web::Data<Arc<Mutex<AppState>>>,
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

    let engine_guard = engine.lock().await;
    HttpResponse::Ok().json(engine_guard.state())
}
