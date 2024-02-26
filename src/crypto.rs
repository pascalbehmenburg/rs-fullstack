use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

pub async fn hash_password(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);

    // argond2id v19 default params
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password, &salt)
        .expect("Failed to hash password.")
        .to_string();

    password_hash
}
