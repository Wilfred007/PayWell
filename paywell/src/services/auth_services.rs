use bcrypt::{hash, verify};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use rand::rngs::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;




#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}


pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error>{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}


pub fn verify_password(hash: &str, password: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed) => Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok(),
        Err(_) => false,
    }
} 


pub fn generate_jwt(user_id: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Valid timestamp")
        .timestamp() as usize;


    let claims = Claims {
        sub: user_id.to_owned(),
        exp,
    };


    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
    
}


