use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct User {
    #[serde(rename = "_id")]
    #[schema(value_type = String)]
    pub id: ObjectId,
    pub username: String,
    pub password: String,
    pub refresh_jti: Option<String>,
}
