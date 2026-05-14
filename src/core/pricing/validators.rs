use mongodb::{Collection, bson::doc};
use serde::Serialize;

use crate::core::pricing::{models::Pricing, payloads::PricingCreatePayload};

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
    pub async fn validate_update_pricing(
        payload: &PricingUpdatePayload,
        pricings: &Collection<Pricing>,
    ) -> Result<(), ValidationErrors> {
        let mut errors = Vec::new();

        if let Some(nom) = &payload.nom
            && nom.len() < 3
        {
            errors.push(ValidationError {
                field: "nom".to_string(),
                message: "Le nom doit contenir au moins 3 caractères".to_string(),
            });
        } else if let Some(nom) = &payload.nom {
            // Check for duplicate name
            let existing_pricing = pricings.find_one(doc! { "nom": nom }).await.unwrap_or(None);

            if existing_pricing.is_some() {
                errors.push(ValidationError {
                    field: "nom".to_string(),
                    message: "Un tarif avec ce nom existe déjà".to_string(),
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

        if let Some(description) = &payload.description
            && description.is_empty()
        {
            errors.push(ValidationError {
                field: "description".to_string(),
                message: "La description doit contenir au moins 1 caractère".to_string(),
            });
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrors { errors })
        }
    }

    pub async fn validate_create_pricing(
        payload: &PricingCreatePayload,
        pricings: &Collection<Pricing>,
    ) -> Result<(), ValidationErrors> {
        let mut errors = Vec::new();

        if payload.nom.len() < 3 {
            errors.push(ValidationError {
                field: "nom".to_string(),
                message: "Le nom doit contenir au moins 3 caractères".to_string(),
            });
        }
        let existing_pricing = pricings
            .find_one(doc! { "nom": payload.nom.clone() })
            .await
            .unwrap_or(None);

        if existing_pricing.is_some() {
            errors.push(ValidationError {
                field: "nom".to_string(),
                message: "Un tarif avec ce nom existe déjà".to_string(),
            });
        }

        if payload.prix < 0.0 {
            errors.push(ValidationError {
                field: "prix".to_string(),
                message: "Le prix doit être positif".to_string(),
            });
        }

        if payload.description.is_empty() {
            errors.push(ValidationError {
                field: "description".to_string(),
                message: "La description doit contenir au moins 1 caractère".to_string(),
            });
        }

        if payload.categorie.is_empty() {
            errors.push(ValidationError {
                field: "categorie".to_string(),
                message: "La catégorie est obligatoire".to_string(),
            });
        }

        if payload.sous_categorie.is_empty() {
            errors.push(ValidationError {
                field: "sous_categorie".to_string(),
                message: "La sous-catégorie est obligatoire".to_string(),
            });
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrors { errors })
        }
    }
}
