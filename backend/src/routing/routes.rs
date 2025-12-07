use axum::{Router, routing::get};
use super::handler::user_handler;
use sqlx::MySqlPool;

pub fn create_routes(pool: MySqlPool) -> Router {
    let pool_for_matches = pool.clone();
    let pool_for_like = pool.clone();

    Router::new()
        .route(
            "/matches",
            get(move || user_handler::get_users(pool_for_matches.clone())),
        )
        .route(
            "/like/:user_id",
            get(move |user_id: String| user_handler::like_user(pool_for_like.clone(), user_id)),
        )
}
