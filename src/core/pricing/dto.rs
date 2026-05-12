use serde::Serialize;

use super::models::{Categorie, Pricing};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PricingResponse {
    pub id: String,
    pub categorie: Categorie,
    pub sous_categorie: String,
    pub nom: String,
    pub description: String,
    pub prix: f64,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

#[derive(Serialize, ToSchema)]
pub struct PricingsResponse {
    pub pricings: Vec<PricingResponse>,
}

impl From<Pricing> for PricingResponse {
    fn from(pricing: Pricing) -> Self {
        Self {
            id: pricing.id.to_hex(),
            categorie: pricing.categorie,
            sous_categorie: pricing.sous_categorie,
            nom: pricing.nom,
            description: pricing.description,
            prix: pricing.prix,
            created_at: pricing.created_at,
            updated_at: pricing.updated_at,
        }
    }
}

impl PricingsResponse {
    pub fn new(pricings: Vec<Pricing>) -> Self {
        Self {
            pricings: pricings.into_iter().map(PricingResponse::from).collect(),
        }
    }
}
