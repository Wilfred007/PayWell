use axum::http::HeaderMap;
use hmac::{Hmac, Mac};
use sha2::Sha512;
use hex::encode;

/// Verifies x-paystack-signature header against request body using secret (HMAC SHA512)
pub fn verify_paystack_signature(headers: &HeaderMap, body: &[u8], secret: &str) -> bool {
    type HmacSha512 = Hmac<Sha512>;
    if let Some(sig) = headers.get("x-paystack-signature") {
        let sig = sig.to_str().unwrap_or("");
        let mut mac = HmacSha512::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
        mac.update(body);
        let result = mac.finalize().into_bytes();
        let expected = encode(result);
        expected == sig
    } else {
        false
    }
}
