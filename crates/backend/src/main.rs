use actix_web::{
    App, HttpServer,
    web::{post, resource},
};
use api::{apply_move, new_game};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(resource("/game").route(post().to(new_game)))
            .service(resource("/game/{id}").route(post().to(apply_move)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
