use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use engine::LocalEngine;
use uuid::Uuid;

pub struct AppState {
    pub games: HashMap<Uuid, LocalEngine>,
}

impl AppState {
    pub fn new() -> Self {
        AppState { games: HashMap::new() }
    }

    pub fn add_game(&mut self, id: Uuid, engine: LocalEngine) {
        self.games.insert(id, engine);
    }

    pub fn get_game(&mut self, id: Uuid) -> Option<&mut LocalEngine> {
        self.games.get_mut(&id)
    }
}
