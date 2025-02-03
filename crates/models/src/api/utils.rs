#[cfg(feature = "ssr")]
pub fn hash_key(key: &str) -> Result<String, argon2::password_hash::Error> {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
        Argon2,
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let key_hash = argon2.hash_password(key.as_bytes(), &salt)?.to_string();
    Ok(PasswordHash::new(&key_hash)?.to_string())
}

#[cfg(feature = "ssr")]
pub fn generate_api_key() -> Result<(String, String, String), argon2::password_hash::Error> {
    use base64::engine::Engine;
    use rand::prelude::*;

    let custom = base64::alphabet::Alphabet::new(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-",
    )
    .unwrap();
    let engine =
        base64::engine::GeneralPurpose::new(&custom, base64::engine::general_purpose::NO_PAD);

    use crate::api::utils::hash_key;

    let mut rng = rand::thread_rng();
    let mut key_bytes = [0u8; 12];
    rng.fill(&mut key_bytes);
    let key_id = engine.encode(key_bytes);

    let mut rng = rand::thread_rng();
    let mut key_bytes = [0u8; 32];
    rng.fill(&mut key_bytes);
    let key = engine.encode(key_bytes);

    let key_hash = hash_key(&key)?;

    Ok((key_id, key, key_hash))
}
