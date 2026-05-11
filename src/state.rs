use mongodb::Collection;

use crate::core::auth::models::User;

#[derive(Clone)]
pub struct AppState {
    pub mongo: mongodb::Client,
    pub secret_store: SecretStore,
    pub started_at: std::time::Instant,
    pub verbose: bool,
}

impl AppState {
    pub fn get_user_collection(&self) -> Collection<User> {
        self.mongo.database("umdr-db").collection("user")
    }
}

#[derive(Clone)]
pub struct SecretStore;

impl SecretStore {
    pub fn get(&self, key: &str) -> Option<String> {
        std::env::var(key).ok()
    }
}
