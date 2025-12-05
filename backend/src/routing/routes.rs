use axum::{Router, routing::get};
use super::handler::{user_handler};
use sqlx::MySqlPool;

pub fn create_routes(pool: MySqlPool) -> Router {
    Router::new()
        .route("/users", get(move || user_handler::get_users(pool.clone())))
}
