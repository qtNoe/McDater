use axum::Json;
use sqlx::MySqlPool;
use sqlx::Row;
use serde_json::json;

pub async fn get_users(pool: MySqlPool) -> Json<Vec<serde_json::Value>> {
    let rows = sqlx::query("SELECT * FROM z_user")
        .fetch_all(&pool)
        .await
        .unwrap();

    let users = rows.into_iter()
        .map(|row| {
            json!({
                "id": row.get::<i64, _>("id"),
                "email": row.get::<String, _>("email"),
            })
        })
        .collect::<Vec<_>>();

    Json(users)
}
