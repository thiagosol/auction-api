use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};
use sqlx::Error;
use sqlx::PgPool;
use sqlx::Row;

use crate::models::condition::Condition;
use crate::models::property::Property;
use crate::models::property::PropertyQueryParams;

pub async fn get_properties(
    pool: web::Data<PgPool>,
    query_params: web::Query<PropertyQueryParams>,
) -> impl Responder {
    let mut query = String::from("SELECT p.external_id AS id, 
            p.external_id,
            m.name as modality,
            p.title, p.appraisal_value, p.value, p.discount, 
            p.type AS property_type, p.situation, p.registration, p.auction_date, 
            p.auction_notice, p.auction_item_number, p.address, p.cep, 
            p.neighborhood, p.registration_link, p.auction_link, p.link, 
            p.created_at, p.modality_id
        FROM property p
        INNER JOIN modality m on m.id = p.modality_id
        WHERE 1=1");

    let mut params: Vec<String> = Vec::new();

    if let Some(nome) = &query_params.nome {
        query.push_str(" AND p.title ILIKE $");
        query.push_str(&(params.len() + 1).to_string());
        params.push(format!("%{}%", nome));
    }

    if let Some(tipo) = &query_params.tipo {
        query.push_str(" AND p.type = $");
        query.push_str(&(params.len() + 1).to_string());
        params.push(tipo.clone());
    }

    if let Some(endereco) = &query_params.endereco {
        query.push_str(" AND p.address ILIKE $");
        query.push_str(&(params.len() + 1).to_string());
        params.push(format!("%{}%", endereco));
    }

    if let Some(bairro) = &query_params.bairro {
        query.push_str(" AND p.neighborhood ILIKE $");
        query.push_str(&(params.len() + 1).to_string());
        params.push(format!("%{}%", bairro));
    }

    if let Some(situacao) = &query_params.situacao {
        query.push_str(" AND p.situation = $");
        query.push_str(&(params.len() + 1).to_string());
        params.push(situacao.clone());
    }

    if let Some(valor_min) = query_params.valor_min {
        query.push_str(&format!(" AND p.value >= {}", valor_min));
    }

    if let Some(valor_max) = query_params.valor_max {
        query.push_str(&format!(" AND p.value <= {}", valor_max));
    }

    if let Some(modalidade) = query_params.modalidade {
        query.push_str(&format!(" AND p.modality_id = {}", modalidade));
    }

    let mut query_exec = sqlx::query(&query);
    for param in params.iter() {
        query_exec = query_exec.bind(param);
    }

    let rows = match query_exec.fetch_all(pool.get_ref()).await {
        Ok(rows) => rows,
        Err(err) => {
            eprintln!("Erro ao buscar propriedades: {:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let mut properties = Vec::new();
    let mut external_ids = Vec::new();

    for row in rows {
        let property = Property {
            id: row.get("id"),
            external_id: row.get("external_id"),
            title: row.get("title"),
            appraisal_value: row.get("appraisal_value"),
            value: row.get("value"),
            discount: row.get("discount"),
            property_type: row.get("property_type"),
            situation: row.get("situation"),
            registration: row.get("registration"),
            auction_date: row.get("auction_date"),
            auction_notice: row.get("auction_notice"),
            auction_item_number: row.get("auction_item_number"),
            address: row.get("address"),
            cep: row.get("cep"),
            neighborhood: row.get("neighborhood"),
            registration_link: row.get("registration_link"),
            auction_link: row.get("auction_link"),
            link: row.get("link"),
            created_at: row.get("created_at"),
            modality_id: row.get("modality_id"),
            modality: row.get("modality"),
            conditions: Some(Vec::new()),
        };

        external_ids.push(property.external_id.clone());
        properties.push(property);
    }

    let conditions_result = get_conditions(pool.get_ref(), &external_ids).await;
    let conditions_map: HashMap<String, Vec<Condition>> = match conditions_result {
        Ok(conditions) => {
            let mut map = HashMap::new();
            for condition in conditions {
                map.entry(condition.external_id.clone())
                    .or_insert_with(Vec::new)
                    .push(condition);
            }
            map
        }
        Err(err) => {
            eprintln!("Erro ao buscar condições: {:?}", err);
            HashMap::new()
        }
    };

    for property in &mut properties {
        property.conditions = Some(
            conditions_map
                .get(&property.external_id)
                .cloned()
                .unwrap_or_default(),
        );
    }

    HttpResponse::Ok().json(properties)
}

async fn get_conditions(pool: &PgPool, external_ids: &[String]) -> Result<Vec<Condition>, Error> {
    if external_ids.is_empty() {
        return Ok(vec![]);
    }

    let placeholders: Vec<String> = external_ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("${}", i + 1))
        .collect();
    let in_clause = placeholders.join(",");

    let query_sql = format!(
        "SELECT c.external_id, c.description 
         FROM condition c 
         WHERE c.external_id IN ({})",
        in_clause
    );

    let mut sql_query = sqlx::query_as::<_, Condition>(&query_sql);

    for external_id in external_ids {
        sql_query = sql_query.bind(external_id);
    }

    sql_query.fetch_all(pool).await
}
