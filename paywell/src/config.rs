use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub paystack_secret: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set"),
            paystack_secret: env::var("PAYSTACK_SECRET").expect("PAYSTACK_SECRET not set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET not set"),
        }
    }
}
