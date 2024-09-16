use crate::db::crud::get_brand;
use crate::models::brand::Brand;
use actix_web::{web, Responder, Result};
use serde::Deserialize;
use serde_json::{json, Value};

async fn fetch_brands(brand_name: String) -> Result<Value> {
    let fetch_res = get_brand(brand_name).await;
    if fetch_res.is_err() {
        let err_msg: String = fetch_res.err().unwrap().to_string();
        let err_response: Value = json!({
            "status": 0,
            "err_msg": err_msg,
            "data": null
        });

        return Ok(err_response);
    }

    let brands: Vec<Brand> = fetch_res.unwrap();

    if brands.is_empty() {
        let err_reponse: Value = json!({
            "status": 0,
            "err_msg": "No brands found",
            "data": null
        });

        return Ok(err_reponse);
    }

    Ok(json!({
        "status": 1,
        "err_msg": null,
        "data": brands
    }))
}

#[derive(Deserialize)]
pub struct Query {
    pub brand_name: String,
}
pub async fn fetch_brands_handler(query: web::Query<Query>) -> impl Responder {
    let name = &query.brand_name;

    match fetch_brands(name.to_string()).await {
        Ok(response) => web::Json(response),
        Err(_) => web::Json(json!({
            "status": 0,
            "err_msg": "Failed to fetch brands",
            "data": null
        })),
    }
}
