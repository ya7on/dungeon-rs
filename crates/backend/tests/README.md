# Backend API Tests

This directory contains integration tests for the dungeon-rs backend HTTP API service.

## API Endpoints Tested

### 1. `POST /game` - Create New Game
Creates a new game instance with a unique UUID and returns the initial game state.

**Response:**
```json
{
  "game_id": "uuid-string",
  "state": {
    "player": {
      "x": 0,
      "y": 0
    }
  }
}
```

### 2. `GET /game/{id}` - Get Game State
Retrieves the current state of a specific game by its UUID.

**Response:**
```json
{
  "player": {
    "x": 0,
    "y": 0
  }
}
```

**Error Cases:**
- `404 Not Found` - Game with specified ID does not exist

### 3. `POST /game/{id}` - Apply Player Move
Applies a player action to a specific game and returns the step result.

**Request Body:**
```json
{
  "Move": "North" // or "South", "East", "West"
}
```

**Response:**
```json
{
  "events": [...],
  "diff": {...}
}
```

**Error Cases:**
- `404 Not Found` - Game with specified ID does not exist
- `400 Bad Request` - Invalid JSON payload

## Test Coverage

The test suite covers the following scenarios:

### Basic Functionality Tests
- `test_new_game_success` - Successfully create a new game
- `test_game_state_success` - Retrieve game state after creation
- `test_apply_move_success` - Apply a valid move to an existing game

### Error Handling Tests
- `test_game_state_not_found` - Handle requests for non-existent games
- `test_apply_move_not_found` - Handle move requests for non-existent games
- `test_apply_move_invalid_json` - Handle malformed JSON in move requests

### Integration Tests
- `test_full_game_flow` - Complete workflow: create game → apply move → check state
- `test_multiple_games` - Create and manage multiple independent games
- `test_multiple_moves_same_game` - Apply multiple sequential moves to one game
- `test_concurrent_games_isolation` - Verify games operate independently

## Running the Tests

To run only the backend tests:
```bash
cargo test --package backend
```

To run all tests in the project:
```bash
cargo test
```

To run tests with output:
```bash
cargo test --package backend -- --nocapture
```

## Test Architecture

The tests use `actix-web::test` framework for HTTP integration testing:

- Each test creates a fresh `AppState` with empty game storage
- Tests use `test::init_service()` to create a test server instance
- HTTP requests are made using `test::TestRequest` builder
- Responses are validated for both status codes and JSON structure

## Dependencies

Test dependencies include:
- `actix-web` with `macros` feature for test utilities
- `tokio` with `macros` and `rt-multi-thread` features for async testing
- `serde_json` for JSON parsing and validation
- `uuid` for game ID generation and validation