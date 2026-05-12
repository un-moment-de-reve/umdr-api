use serde::Serialize;

use super::payloads::PricingUpdatePayload;

const EPSILON: f64 = 1e-10;

#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ValidationErrors {
    pub errors: Vec<ValidationError>,
}

pub struct PricingValidator;

impl PricingValidator {
    pub fn validate_update_pricing(payload: &PricingUpdatePayload) -> Result<(), ValidationErrors> {
        let mut errors = Vec::new();

        if let Some(nom) = &payload.nom {
            if nom.len() < 3 {
                errors.push(ValidationError {
                    field: "nom".to_string(),
                    message: "Le nom doit contenir au moins 3 caractères".to_string(),
                });
            }
        }

        if let Some(prix) = payload.prix {
            if prix < 0.0 {
                errors.push(ValidationError {
                    field: "prix".to_string(),
                    message: "Le prix doit être positif".to_string(),
                });
            } else if ((prix * 100.0).fract()).abs() > EPSILON {
                errors.push(ValidationError {
                    field: "prix".to_string(),
                    message: "Le prix doit avoir au maximum 2 chiffres après la virgule"
                        .to_string(),
                });
            }
        }

        if let Some(description) = &payload.description {
            if description.is_empty() {
                errors.push(ValidationError {
                    field: "description".to_string(),
                    message: "La description doit contenir au moins 1 caractère".to_string(),
                });
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrors { errors })
        }
    }
}
