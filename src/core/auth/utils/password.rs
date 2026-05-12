use std::env;

use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Algorithm, Argon2, Params, PasswordHasher, PasswordVerifier, Version};
use rand_core::OsRng;

use crate::utils::error::AppError;

fn argon2_instance() -> Result<Argon2<'static>, AppError> {
    let memory_cost = env::var("ARGON2_MEMORY_COST")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(19_456);

    let time_cost = env::var("ARGON2_TIME_COST")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(2);

    let parallelism = env::var("ARGON2_PARALLELISM")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(1);

    let params = Params::new(memory_cost, time_cost, parallelism, None)
        .map_err(|_| AppError::internal("Invalid Argon2 parameters"))?;

    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = argon2_instance()?;

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::internal("Failed to hash password"))?
        .to_string();

    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|_| AppError::internal("Invalid stored password hash"))?;

    let argon2 = argon2_instance()?;

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn validate_password_strength(password: &str) -> Result<(), AppError> {
    check_min_length(password, 12)?;
    check_lowercase(password)?;
    check_uppercase(password)?;
    check_digit(password)?;
    check_special_char(password)?;

    Ok(())
}

fn check_min_length(password: &str, min_length: usize) -> Result<(), AppError> {
    if password.len() >= min_length {
        return Ok(());
    }

    Err(AppError::bad_request("Password is too short"))
}

fn check_lowercase(password: &str) -> Result<(), AppError> {
    if password.chars().any(|c| c.is_lowercase()) {
        return Ok(());
    }

    Err(AppError::bad_request(
        "Password must contain at least one lowercase letter",
    ))
}

fn check_uppercase(password: &str) -> Result<(), AppError> {
    if password.chars().any(|c| c.is_uppercase()) {
        return Ok(());
    }

    Err(AppError::bad_request(
        "Password must contain at least one uppercase letter",
    ))
}

fn check_digit(password: &str) -> Result<(), AppError> {
    if password.chars().any(|c| c.is_ascii_digit()) {
        return Ok(());
    }

    Err(AppError::bad_request(
        "Password must contain at least one digit",
    ))
}

fn check_special_char(password: &str) -> Result<(), AppError> {
    if password.chars().any(|c| !c.is_alphanumeric()) {
        return Ok(());
    }

    Err(AppError::bad_request(
        "Password must contain at least one special character",
    ))
}
