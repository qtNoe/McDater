use sqlx::{MySqlPool, Row};

pub async fn find_all_users(pool: &MySqlPool) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query("SELECT uuid FROM user")
        .fetch_all(pool)
        .await?;

    Ok(rows.into_iter().map(|r| r.get("uuid")).collect())
}

pub async fn exists_by_id(pool: &MySqlPool, user_id: &str) -> Result<bool, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*) AS count FROM user WHERE uuid = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    let count: i64 = row.get("count");
    Ok(count > 0)
}
