use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use engine::LocalEngine;
use uuid::Uuid;

/// Application state containing all active game instances.
///
/// This struct maintains a collection of game engines, each identified by a unique UUID.
/// Games are stored in Arc<Mutex<LocalEngine>> to allow safe concurrent access across
/// multiple HTTP requests.
pub struct AppState {
    /// Map of game IDs to their corresponding game engines.
    pub games: HashMap<Uuid, Arc<Mutex<LocalEngine>>>,
}

impl AppState {
    /// Adds a new game to the application state.
    ///
    /// # Parameters
    ///
    /// * `id` - Unique identifier for the game
    /// * `engine` - The game engine instance to be added
    pub fn add_game(&mut self, id: Uuid, engine: LocalEngine) {
        self.games.insert(id, Arc::new(Mutex::new(engine)));
    }

    /// Retrieves a game engine by its ID.
    ///
    /// # Parameters
    ///
    /// * `id` - The unique identifier of the game to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Some(Arc<Mutex<LocalEngine>>)` if the game exists, `None` otherwise.
    /// The returned value is cloned from the internal Arc, allowing multiple references
    /// to the same game engine.
    #[must_use]
    pub fn get_game(&self, id: Uuid) -> Option<Arc<Mutex<LocalEngine>>> {
        self.games.get(&id).cloned()
    }
}
