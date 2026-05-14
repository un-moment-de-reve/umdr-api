use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct PricingUpdatePayload {
    pub nom: Option<String>,
    pub prix: Option<f64>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct PricingCreatePayload {
    pub nom: String,
    pub prix: f64,
    pub description: String,
    pub categorie: String,
    pub sous_categorie: String,
}
