use std::num::NonZeroUsize;

use actix_web::{
    App, HttpServer,
    web::{get, post, resource},
};
use api::{apply_move, game_state, new_game};
use engine::{Engine, LocalTransport};
use lru::LruCache;
use uuid::Uuid;

mod api;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let lru = LruCache::<Uuid, Engine<LocalTransport>>::new(
            NonZeroUsize::new(100).unwrap(),
        );

        App::new()
            .service(resource("/game").route(post().to(new_game)))
            .service(
                resource("/game/{id}")
                    .route(post().to(apply_move))
                    .route(get().to(game_state)),
            )
            .app_data(lru)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
