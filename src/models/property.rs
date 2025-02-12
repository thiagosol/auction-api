use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::types::BigDecimal as SqlxBigDecimal;
use serde_with::{serde_as, DisplayFromStr};

use crate::models::condition::Condition;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub id: String,
    pub external_id: String,
    pub title: String,

    #[serde_as(as = "DisplayFromStr")]
    pub appraisal_value: SqlxBigDecimal,

    #[serde_as(as = "DisplayFromStr")]
    pub value: SqlxBigDecimal,

    pub discount: Option<String>,

    #[serde(rename = "type")]
    pub property_type: String,

    pub situation: String,
    pub registration: Option<String>,
    pub auction_date: Option<NaiveDateTime>,
    pub auction_notice: Option<String>,
    pub auction_item_number: Option<String>,
    pub address: String,
    pub cep: String,
    pub neighborhood: String,
    pub registration_link: Option<String>,
    pub auction_link: Option<String>,
    pub link: String,
    pub created_at: Option<NaiveDateTime>,
    pub modality_id: Option<i32>,
    pub modality: String,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub conditions: Option<Vec<Condition>>,
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyQueryParams {
    #[serde(default, deserialize_with = "empty_as_none")]
    pub nome: Option<String>,

    #[serde(default, deserialize_with = "empty_as_none")]
    pub tipo: Option<String>,

    #[serde(default, deserialize_with = "empty_as_none")]
    pub endereco: Option<String>,

    #[serde(default, deserialize_with = "empty_as_none")]
    pub bairro: Option<String>,

    #[serde(default, deserialize_with = "empty_as_none")]
    pub situacao: Option<String>,

    #[serde(default, deserialize_with = "empty_number_as_none")]
    pub valor_min: Option<f64>,

    #[serde(default, deserialize_with = "empty_number_as_none")]
    pub valor_max: Option<f64>,

    #[serde(default, deserialize_with = "empty_number_as_none")]
    pub modalidade: Option<i32>,
}

fn empty_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.trim().is_empty()))  // Remove strings vazias
}

fn empty_number_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr + serde::de::DeserializeOwned,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) if s.trim().is_empty() => Ok(None),  // 🚀 Se for vazio, retorna `None`
        Some(s) => s.parse::<T>().map(Some).map_err(serde::de::Error::custom),  // 🚀 Se for válido, converte
        None => Ok(None),
    }
}
