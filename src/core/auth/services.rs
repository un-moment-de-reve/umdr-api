use crate::core::auth::dto::TokenResponse;
use crate::core::auth::models::User;
use crate::core::auth::token::{create_tokens, decode_jwt};
use crate::core::auth::utils::jwt::update_refresh_token;
use crate::core::auth::utils::password::verify_password;
use crate::state::SecretStore;
use crate::utils::error::AppError;
use crate::utils::timing::StepTimer;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;

pub async fn login(
    users: Collection<User>,
    secret_store: SecretStore,
    username: String,
    password: String,
    verbose: bool,
) -> Result<TokenResponse, AppError> {
    let mut timer = StepTimer::new(verbose, "auth.login");
    let user = users
        .find_one(doc! { "username": username })
        .await
        .map_err(|_| AppError::database_error())?
        .ok_or(AppError::unauthorized("Invalid credentials"))?;

    timer.step("find_user");

    let is_valid = verify_password(&password, &user.password).unwrap_or(false);
    timer.step("verify_password");
    if is_valid {
        let (access_token, refresh_token, refresh_jti) =
            create_tokens(&user.id.to_string(), &secret_store);
        timer.step("create_tokens");
        update_refresh_token(users, user.id, &refresh_jti)
            .await
            .map_err(|_| AppError::database_error())?;
        timer.step("update_refresh_token");

        Ok(TokenResponse::new(access_token, refresh_token))
    } else {
        Err(AppError::unauthorized("Invalid credentials"))
    }
}

pub async fn refresh_token(
    users: Collection<User>,
    secret_store: SecretStore,
    old_token: &str,
    verbose: bool,
) -> Result<TokenResponse, AppError> {
    let mut timer = StepTimer::new(verbose, "auth.refresh_token");
    let claims = decode_jwt(old_token, &secret_store)
        .map_err(|_| AppError::unauthorized("Invalid token"))?;
    timer.step("decode_jwt");

    if claims.token_type != "refresh" {
        return Err(AppError::unauthorized("Expected refresh token"));
    }

    let old_jti = claims
        .jti
        .as_deref()
        .ok_or_else(|| AppError::unauthorized("Missing jti"))?;

    let user_id = ObjectId::parse_str(&claims.sub)
        .map_err(|_| AppError::unauthorized("Invalid token subject"))?;

    let user = users
        .find_one(doc! { "_id": user_id })
        .await?
        .ok_or_else(|| AppError::unauthorized("Invalid token"))?;
    timer.step("find_user");

    if user.refresh_jti.as_deref() != Some(old_jti) {
        return Err(AppError::unauthorized("Invalid refresh token"));
    }

    let user_id_string = user.id.to_hex();

    let (new_access_token, new_refresh_token, new_refresh_jti) =
        create_tokens(&user_id_string, &secret_store);
    timer.step("create_tokens");

    update_refresh_token(users, user.id, &new_refresh_jti).await?;
    timer.step("update_refresh_token");

    Ok(TokenResponse::new(new_access_token, new_refresh_token))
}
