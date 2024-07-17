use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hashed_password = hash(password, DEFAULT_COST)?;
    Ok(hashed_password)
}

pub fn is_valid(password: &str, hashed_password: &str) -> bool {
    verify(password, hashed_password).unwrap_or(false)
}
