use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::{
    App, HttpServer,
    web::{get, post, resource},
};
use api::{apply_move, game_state, new_game};

use crate::state::AppState;

mod api;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let app_state =
            Arc::new(Mutex::new(AppState { games: HashMap::new() }));

        App::new()
            .service(resource("/game").route(post().to(new_game)))
            .service(
                resource("/game/{id}")
                    .route(post().to(apply_move))
                    .route(get().to(game_state)),
            )
            .app_data(actix_web::web::Data::new(app_state))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
