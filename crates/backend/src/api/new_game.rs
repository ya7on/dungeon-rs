use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, web};
use engine::Engine;
use protocol::State;
use serde::Serialize;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Serialize)]
pub struct NewGameResponse {
    pub game_id: Uuid,
    pub state: State,
}

pub async fn new_game(data: web::Data<Arc<Mutex<AppState>>>) -> HttpResponse {
    let game_id = Uuid::new_v4();

    let state = Engine::new_state();
    let engine = Engine::new_local_game(Arc::new(Mutex::new(state)));
    let state = engine.state();

    let mut guard = data.lock().unwrap();
    guard.add_game(game_id, engine);

    HttpResponse::Ok().json(NewGameResponse { game_id, state })
}
