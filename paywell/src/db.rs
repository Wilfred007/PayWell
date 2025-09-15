use sqlx::mysql::MySqlPool;

pub async fn connect_db(database_url: &str) -> MySqlPool {
    MySqlPool::connect(database_url)
        .await
        .expect("Failed to connect to DB")
}
