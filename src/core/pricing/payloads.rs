use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PricingUpdatePayload {
    pub nom: Option<String>,
    pub prix: Option<f64>,
    pub description: Option<String>,
}