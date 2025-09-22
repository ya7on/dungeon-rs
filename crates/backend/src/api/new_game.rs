use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, web};
use engine::Engine;
use protocol::State;
use serde::Serialize;
use uuid::Uuid;

use crate::state::AppState;

/// Response structure for creating a new game.
#[derive(Serialize)]
pub struct NewGameResponse {
    /// The unique identifier for the newly created game.
    pub game_id: Uuid,
    /// The initial state of the game.
    pub state: State,
}

/// Creates a new game instance.
///
/// This endpoint creates a new game with a unique ID and returns the initial game state.
/// The game is stored in the application state and can be accessed using the returned ID.
///
/// # Returns
///
/// Returns a JSON response containing the game ID and initial state.
pub async fn new_game(data: web::Data<Arc<Mutex<AppState>>>) -> HttpResponse {
    let game_id = Uuid::new_v4();

    let state = Engine::new_state();
    let engine = Engine::new_local_game(Arc::new(Mutex::new(state)));
    let state = engine.state();

    let Ok(mut guard) = data.lock() else {
        return HttpResponse::InternalServerError()
            .json("Failed to acquire lock");
    };
    guard.add_game(game_id, engine);

    HttpResponse::Ok().json(NewGameResponse { game_id, state })
}
