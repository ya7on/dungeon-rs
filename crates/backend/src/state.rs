use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use engine::LocalEngine;
use uuid::Uuid;

pub struct AppState {
    pub games: HashMap<Uuid, Arc<Mutex<LocalEngine>>>,
}

impl AppState {
    pub fn add_game(&mut self, id: Uuid, engine: LocalEngine) {
        self.games.insert(id, Arc::new(Mutex::new(engine)));
    }

    pub fn get_game(&self, id: Uuid) -> Option<Arc<Mutex<LocalEngine>>> {
        self.games.get(&id).cloned()
    }
}
