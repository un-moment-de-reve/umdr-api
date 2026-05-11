use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub password: String,
    pub refresh_jti: Option<String>,
}
