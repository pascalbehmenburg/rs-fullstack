use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use secrecy::{ExposeSecret, Secret};

/// Hashes a secret password using the Argon2id v19 algorithm.
/// The hash is returned in a secret PHC string see https://docs.rs/password-hash/0.5.0/password_hash/struct.PasswordHash.html
/// for more information. This is done so that we can store the hash in the database without exposing the password and retaining
/// the ability to verify the password later.
#[tracing::instrument(name = "password hashing", skip(password))]
pub fn hash_password(password: Secret<String>) -> Secret<String> {
    Secret::new(
        Argon2::default()
            .hash_password(
                password.expose_secret().as_bytes(),
                &SaltString::generate(&mut OsRng),
            )
            .expect("failed to hash password")
            .to_string(),
    )
}
