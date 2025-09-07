use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::state::AppState;

pub async fn game_state(
    path: web::Path<(Uuid)>,
    data: web::Data<Arc<Mutex<AppState>>>,
) -> HttpResponse {
    let game_id = path.into_inner();

    let mut guard = data.lock().unwrap();
    let engine = guard.get_game(game_id).unwrap();

    HttpResponse::Ok().json(engine.state())
}
