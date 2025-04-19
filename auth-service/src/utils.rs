use argon2::{self, Config};

pub fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let salt = b"randomsalt";
    argon2::hash_encoded(password.as_bytes(), salt, &Config::default())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password.as_bytes())
}
