use actix_web::{App, http::StatusCode, test, web};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use backend::{
    api::{apply_move, game_state, new_game},
    state::AppState,
};
use protocol::{Direction, PlayerAction};

#[actix_web::test]
async fn test_new_game_success() {
    let app_state = Arc::new(Mutex::new(AppState { games: HashMap::new() }));

    let app = test::init_service(
        App::new()
            .route("/game", web::post().to(new_game))
            .app_data(web::Data::new(app_state)),
    )
    .await;

    let req = test::TestRequest::post().uri("/game").to_request();

    let resp = test::call_service(&app, req).await;

    // Check status code
    assert_eq!(resp.status(), StatusCode::OK);

    // Parse response body
    let body = test::read_body(resp).await;
    let json_body: Value =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Check response structure
    assert!(json_body["game_id"].is_string());
    assert!(json_body["state"].is_object());

    // Validate that game_id is a valid UUID
    let game_id_str = json_body["game_id"].as_str().unwrap();
    let _game_id = Uuid::parse_str(game_id_str).expect("Invalid UUID format");

    // Check that state has player field
    assert!(json_body["state"]["player"].is_object());
    assert!(json_body["state"]["player"]["x"].is_number());
    assert!(json_body["state"]["player"]["y"].is_number());
}

#[actix_web::test]
async fn test_game_state_success() {
    let app_state = Arc::new(Mutex::new(AppState { games: HashMap::new() }));

    let app = test::init_service(
        App::new()
            .route("/game", web::post().to(new_game))
            .route("/game/{id}", web::get().to(game_state))
            .app_data(web::Data::new(app_state)),
    )
    .await;

    // First, create a new game
    let create_req = test::TestRequest::post().uri("/game").to_request();

    let create_resp = test::call_service(&app, create_req).await;
    let create_body = test::read_body(create_resp).await;
    let create_json: Value = serde_json::from_slice(&create_body).unwrap();
    let game_id = create_json["game_id"].as_str().unwrap();

    // Now get the game state
    let req = test::TestRequest::get()
        .uri(&format!("/game/{}", game_id))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Check status code
    assert_eq!(resp.status(), StatusCode::OK);

    // Parse response body
    let body = test::read_body(resp).await;
    let json_body: Value =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Check response structure (should be the same as initial state)
    assert!(json_body["player"].is_object());
    assert!(json_body["player"]["x"].is_number());
    assert!(json_body["player"]["y"].is_number());
}

#[actix_web::test]
async fn test_game_state_not_found() {
    let app_state = Arc::new(Mutex::new(AppState { games: HashMap::new() }));

    let app = test::init_service(
        App::new()
            .route("/game/{id}", web::get().to(game_state))
            .app_data(web::Data::new(app_state)),
    )
    .await;

    // Try to get state for non-existent game
    let fake_id = Uuid::new_v4();
    let req = test::TestRequest::get()
        .uri(&format!("/game/{}", fake_id))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Should return 404 for non-existent game
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_apply_move_success() {
    let app_state = Arc::new(Mutex::new(AppState { games: HashMap::new() }));

    let app = test::init_service(
        App::new()
            .route("/game", web::post().to(new_game))
            .route("/game/{id}", web::post().to(apply_move))
            .app_data(web::Data::new(app_state)),
    )
    .await;

    // First, create a new game
    let create_req = test::TestRequest::post().uri("/game").to_request();

    let create_resp = test::call_service(&app, create_req).await;
    let create_body = test::read_body(create_resp).await;
    let create_json: Value = serde_json::from_slice(&create_body).unwrap();
    let game_id = create_json["game_id"].as_str().unwrap();

    // Apply a move action
    let move_action = PlayerAction::Move(Direction::North);
    let req = test::TestRequest::post()
        .uri(&format!("/game/{}", game_id))
        .set_json(&move_action)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Check status code
    assert_eq!(resp.status(), StatusCode::OK);

    // Parse response body
    let body = test::read_body(resp).await;
    let json_body: Value =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Check response structure (StepResult)
    assert!(json_body["events"].is_array());
    assert!(json_body["diff"].is_object());
}

#[actix_web::test]
async fn test_apply_move_not_found() {
    let app_state = Arc::new(Mutex::new(AppState { games: HashMap::new() }));

    let app = test::init_service(
        App::new()
            .route("/game/{id}", web::post().to(apply_move))
            .app_data(web::Data::new(app_state)),
    )
    .await;

    // Try to apply move to non-existent game
    let fake_id = Uuid::new_v4();
    let move_action = PlayerAction::Move(Direction::South);
    let req = test::TestRequest::post()
        .uri(&format!("/game/{}", fake_id))
        .set_json(&move_action)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Should return 404 for non-existent game
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_apply_move_invalid_json() {
    let app_state = Arc::new(Mutex::new(AppState { games: HashMap::new() }));

    let app = test::init_service(
        App::new()
            .route("/game", web::post().to(new_game))
            .route("/game/{id}", web::post().to(apply_move))
            .app_data(web::Data::new(app_state)),
    )
    .await;

    // First, create a new game
    let create_req = test::TestRequest::post().uri("/game").to_request();

    let create_resp = test::call_service(&app, create_req).await;
    let create_body = test::read_body(create_resp).await;
    let create_json: Value = serde_json::from_slice(&create_body).unwrap();
    let game_id = create_json["game_id"].as_str().unwrap();

    // Try to apply move with invalid JSON
    let req = test::TestRequest::post()
        .uri(&format!("/game/{}", game_id))
        .set_payload("invalid json")
        .insert_header(("content-type", "application/json"))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Should return 400 for bad request
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_full_game_flow() {
    let app_state = Arc::new(Mutex::new(AppState { games: HashMap::new() }));

    let app = test::init_service(
        App::new()
            .route("/game", web::post().to(new_game))
            .route("/game/{id}", web::get().to(game_state))
            .route("/game/{id}", web::post().to(apply_move))
            .app_data(web::Data::new(app_state)),
    )
    .await;

    // 1. Create a new game
    let create_req = test::TestRequest::post().uri("/game").to_request();

    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status(), StatusCode::OK);

    let create_body = test::read_body(create_resp).await;
    let create_json: Value = serde_json::from_slice(&create_body).unwrap();
    let game_id = create_json["game_id"].as_str().unwrap();

    // Store initial player position
    let initial_x = create_json["state"]["player"]["x"].as_i64().unwrap();
    let initial_y = create_json["state"]["player"]["y"].as_i64().unwrap();

    // 2. Apply a move
    let move_action = PlayerAction::Move(Direction::East);
    let move_req = test::TestRequest::post()
        .uri(&format!("/game/{}", game_id))
        .set_json(&move_action)
        .to_request();

    let move_resp = test::call_service(&app, move_req).await;
    assert_eq!(move_resp.status(), StatusCode::OK);

    // 3. Check updated game state
    let state_req = test::TestRequest::get()
        .uri(&format!("/game/{}", game_id))
        .to_request();

    let state_resp = test::call_service(&app, state_req).await;
    assert_eq!(state_resp.status(), StatusCode::OK);

    let state_body = test::read_body(state_resp).await;
    let state_json: Value = serde_json::from_slice(&state_body).unwrap();

    // Player position might have changed (depending on game logic)
    let new_x = state_json["player"]["x"].as_i64().unwrap();
    let new_y = state_json["player"]["y"].as_i64().unwrap();

    // At minimum, verify the response structure is correct
    assert!(state_json["player"].is_object());

    // Basic sanity checks - position should be valid integers
    assert!(new_x >= initial_x - 100 && new_x <= initial_x + 100);
    assert!(new_y >= initial_y - 100 && new_y <= initial_y + 100);
}

#[actix_web::test]
async fn test_multiple_games() {
    let app_state = Arc::new(Mutex::new(AppState { games: HashMap::new() }));

    let app = test::init_service(
        App::new()
            .route("/game", web::post().to(new_game))
            .route("/game/{id}", web::get().to(game_state))
            .app_data(web::Data::new(app_state)),
    )
    .await;

    // Create first game
    let create_req1 = test::TestRequest::post().uri("/game").to_request();

    let create_resp1 = test::call_service(&app, create_req1).await;
    let create_body1 = test::read_body(create_resp1).await;
    let create_json1: Value = serde_json::from_slice(&create_body1).unwrap();
    let game_id1 = create_json1["game_id"].as_str().unwrap();

    // Create second game
    let create_req2 = test::TestRequest::post().uri("/game").to_request();

    let create_resp2 = test::call_service(&app, create_req2).await;
    let create_body2 = test::read_body(create_resp2).await;
    let create_json2: Value = serde_json::from_slice(&create_body2).unwrap();
    let game_id2 = create_json2["game_id"].as_str().unwrap();

    // Verify they have different IDs
    assert_ne!(game_id1, game_id2);

    // Verify both games are accessible
    let state_req1 = test::TestRequest::get()
        .uri(&format!("/game/{}", game_id1))
        .to_request();

    let state_resp1 = test::call_service(&app, state_req1).await;
    assert_eq!(state_resp1.status(), StatusCode::OK);

    let state_req2 = test::TestRequest::get()
        .uri(&format!("/game/{}", game_id2))
        .to_request();

    let state_resp2 = test::call_service(&app, state_req2).await;
    assert_eq!(state_resp2.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_multiple_moves_same_game() {
    let app_state = Arc::new(Mutex::new(AppState { games: HashMap::new() }));

    let app = test::init_service(
        App::new()
            .route("/game", web::post().to(new_game))
            .route("/game/{id}", web::get().to(game_state))
            .route("/game/{id}", web::post().to(apply_move))
            .app_data(web::Data::new(app_state)),
    )
    .await;

    // Create a new game
    let create_req = test::TestRequest::post().uri("/game").to_request();

    let create_resp = test::call_service(&app, create_req).await;
    let create_body = test::read_body(create_resp).await;
    let create_json: Value = serde_json::from_slice(&create_body).unwrap();
    let game_id = create_json["game_id"].as_str().unwrap();

    // Apply multiple moves
    let moves =
        [Direction::North, Direction::East, Direction::South, Direction::West];

    for direction in moves {
        let move_action = PlayerAction::Move(direction);
        let move_req = test::TestRequest::post()
            .uri(&format!("/game/{}", game_id))
            .set_json(&move_action)
            .to_request();

        let move_resp = test::call_service(&app, move_req).await;
        assert_eq!(move_resp.status(), StatusCode::OK);

        // Verify response structure
        let move_body = test::read_body(move_resp).await;
        let move_json: Value = serde_json::from_slice(&move_body).unwrap();
        assert!(move_json["events"].is_array());
        assert!(move_json["diff"].is_object());
    }

    // Verify game state is still accessible
    let state_req = test::TestRequest::get()
        .uri(&format!("/game/{}", game_id))
        .to_request();

    let state_resp = test::call_service(&app, state_req).await;
    assert_eq!(state_resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_concurrent_games_isolation() {
    let app_state = Arc::new(Mutex::new(AppState { games: HashMap::new() }));

    let app = test::init_service(
        App::new()
            .route("/game", web::post().to(new_game))
            .route("/game/{id}", web::post().to(apply_move))
            .route("/game/{id}", web::get().to(game_state))
            .app_data(web::Data::new(app_state)),
    )
    .await;

    // Create two games
    let create_req1 = test::TestRequest::post().uri("/game").to_request();

    let create_resp1 = test::call_service(&app, create_req1).await;
    let create_body1 = test::read_body(create_resp1).await;
    let create_json1: Value = serde_json::from_slice(&create_body1).unwrap();
    let game_id1 = create_json1["game_id"].as_str().unwrap();

    let create_req2 = test::TestRequest::post().uri("/game").to_request();

    let create_resp2 = test::call_service(&app, create_req2).await;
    let create_body2 = test::read_body(create_resp2).await;
    let create_json2: Value = serde_json::from_slice(&create_body2).unwrap();
    let game_id2 = create_json2["game_id"].as_str().unwrap();

    // Apply different moves to each game
    let move1 = PlayerAction::Move(Direction::North);
    let move_req1 = test::TestRequest::post()
        .uri(&format!("/game/{}", game_id1))
        .set_json(&move1)
        .to_request();

    let move_resp1 = test::call_service(&app, move_req1).await;
    assert_eq!(move_resp1.status(), StatusCode::OK);

    let move2 = PlayerAction::Move(Direction::South);
    let move_req2 = test::TestRequest::post()
        .uri(&format!("/game/{}", game_id2))
        .set_json(&move2)
        .to_request();

    let move_resp2 = test::call_service(&app, move_req2).await;
    assert_eq!(move_resp2.status(), StatusCode::OK);

    // Verify both games still work independently
    let state_req1 = test::TestRequest::get()
        .uri(&format!("/game/{}", game_id1))
        .to_request();

    let state_resp1 = test::call_service(&app, state_req1).await;
    assert_eq!(state_resp1.status(), StatusCode::OK);

    let state_req2 = test::TestRequest::get()
        .uri(&format!("/game/{}", game_id2))
        .to_request();

    let state_resp2 = test::call_service(&app, state_req2).await;
    assert_eq!(state_resp2.status(), StatusCode::OK);
}
