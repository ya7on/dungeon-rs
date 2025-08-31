use actix_web::HttpResponse;

pub async fn apply_move() -> HttpResponse {
    HttpResponse::Ok().body("Hello World!")
}
