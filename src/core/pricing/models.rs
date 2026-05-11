use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Categorie {
    Epilation,
    SoinVisage,
    SoinCorps,
    MainsEtPieds,
    MiseEnBeaute,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pricing {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub categorie: Categorie,
    pub sous_categorie: String,
    pub nom: String,
    pub description: String,
    pub prix: f64,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

impl FromStr for Categorie {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "epilation" => Ok(Categorie::Epilation),
            "soin_visage" => Ok(Categorie::SoinVisage),
            "soin_corps" => Ok(Categorie::SoinCorps),
            "mains_et_pieds" => Ok(Categorie::MainsEtPieds),
            "mise_en_beaute" => Ok(Categorie::MiseEnBeaute),
            _ => Err(()),
        }
    }
}