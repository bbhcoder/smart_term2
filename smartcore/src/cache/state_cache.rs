use std::sync::{Arc, RwLock};
use db::models::SessionState;

#[derive(Clone)]
pub struct StateCache {
    state: Arc<RwLock<SessionState>>,
}

impl StateCache {
    pub fn new(initial: SessionState) -> Self {
        Self {
            state: Arc::new(RwLock::new(initial)),
        }
    }

    pub fn get(&self) -> SessionState {
        self.state.read().unwrap().clone()
    }

    pub fn update(&self, new_state: SessionState) {
        *self.state.write().unwrap() = new_state;
    }
}
