use std::env;

use mongodb::{
    Database,
    bson::{Document, doc, oid::ObjectId},
};

use crate::{core::auth::models::User, core::auth::utils::password::hash_password};

pub async fn seed_default_user(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let users = db.collection::<User>("user");

    let existing_user_count = users.count_documents(doc! {}).await?;

    if existing_user_count > 0 {
        return Ok(());
    }

    let username = env::var("DEFAULT_ADMIN_USERNAME")
        .map_err(|_| "DEFAULT_ADMIN_USERNAME is missing in .env")?;

    let password = env::var("DEFAULT_ADMIN_PASSWORD")
        .map_err(|_| "DEFAULT_ADMIN_PASSWORD is missing in .env")?;

    let password_hash =
        hash_password(&password).map_err(|_| "Failed to hash default admin password")?;

    let user = User {
        id: ObjectId::new(),
        username,
        password: password_hash,
        refresh_jti: None,
    };

    users.insert_one(user).await?;

    Ok(())
}

pub async fn seed_default_pricing(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let pricing_path = "pricings_seed.json";

    if !std::path::Path::new(pricing_path).exists() {
        return Ok(());
    }

    let pricing_data = std::fs::read_to_string(pricing_path)?;

    let default_pricings: Vec<Document> = serde_json::from_str(&pricing_data)?;

    let pricing = db.collection::<Document>("pricing");

    let existing_pricing_count = pricing.count_documents(doc! {}).await?;

    if existing_pricing_count > 0 {
        println!("Default pricing seed skipped: at least one pricing entry already exists");
        return Ok(());
    }

    if default_pricings.is_empty() {
        return Ok(());
    }

    pricing.insert_many(default_pricings).await?;

    Ok(())
}
