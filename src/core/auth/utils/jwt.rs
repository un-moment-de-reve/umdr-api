use crate::core::auth::models::User;
use crate::utils::error::AppError;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};

pub async fn is_refresh_valid(mongodb_client: &Client, user_id: &str, jti: &str) -> bool {
    let collection = mongodb_client
        .database("umdr-db")
        .collection::<User>("user");

    let user = collection
        .find_one(doc! { "_id": ObjectId::parse_str(user_id).unwrap() })
        .await
        .ok()
        .flatten();

    match user {
        Some(u) => u.refresh_jti.as_deref() == Some(jti),
        None => false,
    }
}

pub async fn update_refresh_token(
    users: Collection<User>,
    user_id: ObjectId,
    new_jti: &str,
) -> Result<(), AppError> {
    let result = users
        .update_one(
            doc! { "_id": user_id },
            doc! { "$set": { "refresh_jti": new_jti } },
        )
        .await?;

    if result.matched_count == 0 {
        return Err(AppError::not_found("User not found"));
    }

    Ok(())
}
