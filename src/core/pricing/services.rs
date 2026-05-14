use chrono::Utc;
use mongodb::{
    Collection,
    bson::{Document, doc, oid::ObjectId},
};

use crate::{
    core::pricing::{dto::PricingDeleteResponse, payloads::PricingCreatePayload},
    utils::{error::AppError, timing::StepTimer},
};

use super::{
    dto::{PricingResponse, PricingsResponse},
    models::Pricing,
    payloads::PricingUpdatePayload,
    validators::PricingValidator,
};

pub async fn get_pricings(
    pricings: Collection<Pricing>,
    verbose: bool,
) -> Result<PricingsResponse, AppError> {
    let mut timer = StepTimer::new(verbose, "pricing.update_pricing");

    let mut cursor = pricings.find(doc! {}).await?;
    timer.step("fetch_pricings");

    let mut results = Vec::new();

    while cursor.advance().await? {
        let pricing = cursor.deserialize_current()?;
        results.push(pricing);
    }

    Ok(PricingsResponse::new(results))
}

pub async fn update_pricing(
    pricings: Collection<Pricing>,
    id: ObjectId,
    payload: PricingUpdatePayload,
    verbose: bool,
) -> Result<PricingResponse, AppError> {
    let mut timer = StepTimer::new(verbose, "pricing.update_pricing");
    if let Err(validation_errors) =
        PricingValidator::validate_update_pricing(&payload, &pricings).await
    {
        return Err(AppError::validation(
            "Invalid pricing update request",
            validation_errors,
        ));
    }
    timer.step("validate_payload");

    let mut update_doc = Document::new();

    if let Some(nom) = payload.nom {
        update_doc.insert("nom", nom);
    }

    if let Some(prix) = payload.prix {
        update_doc.insert("prix", prix);
    }

    if let Some(description) = payload.description {
        update_doc.insert("description", description);
    }

    if update_doc.is_empty() {
        return Err(AppError::bad_request(
            "Please provide at least one field to update the pricing",
        ));
    }

    update_doc.insert("updated_at", Utc::now().timestamp());

    let result = pricings
        .update_one(doc! { "_id": id }, doc! { "$set": update_doc })
        .await?;

    timer.step("update_pricing");

    if result.matched_count == 0 {
        return Err(AppError::not_found("Pricing not found"));
    }

    let pricing = pricings
        .find_one(doc! { "_id": id })
        .await?
        .ok_or_else(|| AppError::not_found("Pricing not found"))?;

    timer.step("find_pricing");

    Ok(PricingResponse::from(pricing))
}

pub async fn delete_pricing(
    pricings: Collection<Pricing>,
    id: ObjectId,
    verbose: bool,
) -> Result<PricingDeleteResponse, AppError> {
    let mut timer = StepTimer::new(verbose, "pricing.delete_pricing");

    let result = pricings.delete_one(doc! { "_id": id }).await?;
    timer.step("delete_pricing");

    if result.deleted_count == 0 {
        return Err(AppError::not_found("Pricing not found"));
    }

    Ok(PricingDeleteResponse::from(id))
}

pub async fn create_pricing(
    pricings: Collection<Pricing>,
    payload: PricingCreatePayload,
    verbose: bool,
) -> Result<PricingResponse, AppError> {
    let mut timer = StepTimer::new(verbose, "pricing.create_pricing");
    payload
        .categorie
        .parse::<super::models::Categorie>()
        .map_err(|_| {
            AppError::validation(
                "Invalid category",
                vec![super::validators::ValidationError {
                    field: "categorie".to_string(),
                    message: format!("La catégorie '{}' n'est pas valide", payload.categorie),
                }],
            )
        })?;
    payload.sous_categorie.parse::<String>().map_err(|_| {
        AppError::validation(
            "Invalid sub-category",
            vec![super::validators::ValidationError {
                field: "sous_categorie".to_string(),
                message: format!(
                    "La sous-catégorie '{}' n'est pas valide",
                    payload.sous_categorie
                ),
            }],
        )
    })?;
    if let Err(validation_errors) =
        PricingValidator::validate_create_pricing(&payload, &pricings).await
    {
        return Err(AppError::validation(
            "Invalid pricing creation request",
            validation_errors,
        ));
    }
    timer.step("validate_payload");

    let new_pricing = Pricing {
        id: ObjectId::new(),
        nom: payload.nom,
        prix: payload.prix,
        description: payload.description,
        categorie: payload.categorie.parse().unwrap_or_else(|_| {
            panic!("Invalid category: {}", payload.categorie);
        }),
        sous_categorie: payload.sous_categorie,
        created_at: Utc::now().timestamp(),
        updated_at: None,
    };

    pricings.insert_one(&new_pricing).await?;
    timer.step("insert_pricing");

    Ok(PricingResponse::from(new_pricing))
}
