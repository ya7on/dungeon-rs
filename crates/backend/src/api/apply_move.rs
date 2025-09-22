use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, web};
use protocol::PlayerAction;
use uuid::Uuid;

use crate::state::AppState;

pub async fn apply_move(
    path: web::Path<Uuid>,
    data: web::Data<Arc<Mutex<AppState>>>,
    json: web::Json<PlayerAction>,
) -> HttpResponse {
    let game_id = path.into_inner();

    let engine = {
        let guard = data.lock().unwrap();
        guard.get_game(game_id).unwrap()
    };

    let mut engine_guard = engine.lock().await;
    let result = engine_guard.apply_step(json.into_inner()).await.unwrap();

    HttpResponse::Ok().json(result)
}
