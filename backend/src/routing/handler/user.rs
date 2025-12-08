use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;
use sqlx::MySqlPool;

use crate::service::user_service;


/// GET /matches
pub async fn get_users(State(pool): State<MySqlPool>) -> Json<Vec<serde_json::Value>> {
    let ids = user_service::get_all_user_ids(&pool).await;

    let payload = ids
        .into_iter()
        .map(|id| json!({ "id": id }))
        .collect::<Vec<_>>();

    Json(payload)
}


/// GET /like/:user_id
pub async fn like_user(
    State(pool): State<MySqlPool>,
    Path(user_id): Path<String>,
) -> Json<serde_json::Value> {

    if !user_service::user_exists(&pool, &user_id).await {
        return Json(json!({
            "status": "error",
            "message": format!("User {} does not exist!", user_id)
        }));
    }

    // später: like speichern
    // user_service::like_user(&pool, &user_id).await?;

    Json(json!({
        "status": "success",
        "message": format!("User {} liked!", user_id)
    }))
}


/// GET /dislike/:user_id
pub async fn dislike_user(
    State(pool): State<MySqlPool>,
    Path(user_id): Path<String>,
) -> Json<serde_json::Value> {

    if !user_service::user_exists(&pool, &user_id).await {
        return Json(json!({
            "status": "error",
            "message": format!("User {} does not exist!", user_id)
        }));
    }

    Json(json!({
        "status": "success",
        "message": format!("User {} disliked!", user_id)
    }))
}


/// GET /block/:user_id
pub async fn block_user(
    State(pool): State<MySqlPool>,
    Path(user_id): Path<String>,
) -> Json<serde_json::Value> {

    if !user_service::user_exists(&pool, &user_id).await {
        return Json(json!({
            "status": "error",
            "message": format!("User {} does not exist!", user_id)
        }));
    }

    Json(json!({
        "status": "success",
        "message": format!("User {} blocked!", user_id)
    }))
}
