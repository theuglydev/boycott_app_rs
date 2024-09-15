use crate::db::crud::add_brands;
use crate::models::brand::Brand;
use crate::scrapers::boycott_thewitness;
use crate::scrapers::boycotzionism;
use crate::scrapers::ethical_consumer;
use crate::scrapers::thewitness_apis;
use actix_web::{web, Responder, Result};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct QueryParams {
    pub target: String,
}

pub async fn scrape_brands(target: &str) -> Result<Value> {
    let mut scraped_brands: Vec<Brand> = Vec::new();

    match target {
        "boycotzionism" => {
            let scraping_res = scrape_boycott_zionism().await;
            if scraping_res.is_err() {
                let err_msg: String = scraping_res.err().unwrap().to_string();
                let err_response: Value = json!({
                    "status": 0,
                    "err_msg": err_msg,
                    "data": null
                });

                return Ok(err_response);
            }

            scraped_brands = scraping_res.unwrap();
        }
        "thewitness" => {
            let scraping_res = scrape_thewitness().await;
            if scraping_res.is_err() {
                let err_msg: String = scraping_res.err().unwrap().to_string();
                let err_response: Value = json!({
                    "status": 0,
                    "err_msg": err_msg,
                    "data": null
                });

                return Ok(err_response);
            }

            scraped_brands = scraping_res.unwrap();
        }
        "ethicalconsumer" => {
            let scraping_res = scrape_ethical_consumer().await;
            if scraping_res.is_err() {
                let err_msg: String = scraping_res.err().unwrap().to_string();
                let err_response: Value = json!({
                    "status": 0,
                    "err_msg": err_msg,
                    "data": null
                });

                return Ok(err_response);
            }

            scraped_brands = scraping_res.unwrap();
        }
        _ => {
            let err_response: Value = json!({
               "status": 0,
               "err_msg": "Target param is either missing, typo, or does not exist",
               "data": null
            });

            return Ok(err_response);
        }
    }

    let db_add_res = add_brands(scraped_brands).await;
    if db_add_res.is_err() {
        let err_msg: String = db_add_res.err().unwrap().to_string();
        let err_response: Value = json!({
            "status": 0,
            "err_msg": err_msg,
            "data": null
        });

        return Ok(err_response);
    }

    let response: Value = json!({
        "status": 1,
        "err_msg": null,
        "data": "Data saved in db successfully"
    });

    Ok(response)
}

async fn scrape_boycott_zionism() -> Result<Vec<Brand>, Box<dyn std::error::Error>> {
    let mut brands: Vec<Brand> = Vec::new();

    let boycotzionism_scr_res = boycotzionism::init().await;
    if boycotzionism_scr_res.is_err() {
        let err_msg: String = boycotzionism_scr_res.err().unwrap().to_string();
        return Err(err_msg.into());
    }

    brands = boycotzionism_scr_res.unwrap();

    Ok(brands)
}

async fn scrape_thewitness() -> Result<Vec<Brand>, Box<dyn std::error::Error>> {
    let mut brands: Vec<Brand> = Vec::new();

    let thewitness_page_numbers_res = boycott_thewitness::init().await;
    if thewitness_page_numbers_res.is_err() {
        let err_msg: String = thewitness_page_numbers_res.err().unwrap().to_string();
        return Err(err_msg.into());
    }
    let page_numbers = thewitness_page_numbers_res.unwrap();
    let thewitness_scraping_res =
        thewitness_apis::get_data_from_thewitness_apis(page_numbers).await;
    if thewitness_scraping_res.is_err() {
        let err_msg: String = thewitness_scraping_res.err().unwrap().to_string();
        return Err(err_msg.into());
    }
    brands = thewitness_scraping_res.unwrap();

    Ok(brands)
}

async fn scrape_ethical_consumer() -> Result<Vec<Brand>, Box<dyn std::error::Error>> {
    let mut brands: Vec<Brand> = Vec::new();

    let ethical_consumer_res = ethical_consumer::init().await;
    if ethical_consumer_res.is_err() {
        let err_msg = ethical_consumer_res.err().unwrap().to_string();
        return Err(err_msg.into());
    }
    brands = ethical_consumer_res.unwrap();

    Ok(brands)
}

pub async fn scrape_brands_handler(query: web::Query<QueryParams>) -> impl Responder {
    let target = &query.target;

    match scrape_brands(target).await {
        Ok(response) => web::Json(response),
        Err(_) => web::Json(json!({
            "status": 0,
            "err_msg": "Failed to scrape categories",
            "data": null
        })),
    }
}
