use actix_web::HttpResponse;

pub async fn new_game() -> HttpResponse {
    HttpResponse::Ok().body("Hello World!")
}
