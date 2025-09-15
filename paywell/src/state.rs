use sqlx::MySqlPool;
use reqwest::Client;



#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub http: Client,
    pub paystack_secret: String,
    pub paystack_base_url: String,
    pub jwt_secret: String,
}