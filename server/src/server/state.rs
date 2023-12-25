use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<Client>,
}
