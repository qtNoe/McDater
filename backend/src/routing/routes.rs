use axum::{Router, routing::get};
use sqlx::MySqlPool;

use crate::routing::handler::user::*;


pub fn create_routes() -> Router<MySqlPool> {

    Router::new()
        .route("/matches", get(get_users))
        .route("/like/:user_id", get(like_user))
        .route("/dislike/:user_id", get(dislike_user))
        .route("/block/:user_id", get(block_user))

}
