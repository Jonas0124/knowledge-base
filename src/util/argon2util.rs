use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, PasswordHash, SaltString};

pub fn generate(text: &str) -> String {
    // 生成随机 Salt
    let salt = SaltString::generate(&mut OsRng);

    // 使用 Argon2 进行哈希
    let argon2 = Argon2::default();
    argon2.hash_password(text.as_bytes(), &salt).unwrap().to_string()
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(password_hash).unwrap();
    argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
}