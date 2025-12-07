use axum::Json;
use sqlx::MySqlPool;
use sqlx::Row;
use serde_json::json;

pub async fn get_users(pool: MySqlPool) -> Json<Vec<serde_json::Value>> {
    let rows = sqlx::query("SELECT * FROM user")
        .fetch_all(&pool)
        .await
        .unwrap();

    let users = rows.into_iter()
        .map(|row| {
            json!({
                "id": row.get::<String, _>("uuid"),
            })
        })
        .collect::<Vec<_>>();

    Json(users)
}

pub async fn like_user(pool: MySqlPool, user_id: String) -> Json<serde_json::Value> {
    Json(json!({
        "status": "success",
        "message": format!("User with ID {} liked!", user_id)
    }))
}