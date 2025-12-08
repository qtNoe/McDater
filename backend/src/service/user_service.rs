use sqlx::MySqlPool;
use crate::repository::user_repository;

pub async fn user_exists(pool: &MySqlPool, id: &str) -> bool {
    user_repository::exists_by_id(pool, id)
        .await
        .unwrap_or(false)
}

pub async fn get_all_user_ids(pool: &MySqlPool) -> Vec<String> {
    user_repository::find_all_users(pool)
        .await
        .unwrap_or_default()
}
