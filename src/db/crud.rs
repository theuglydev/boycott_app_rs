use crate::models::brand::Brand;
use futures::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{doc, Regex},
    options::ClientOptions,
    Client, Collection,
};
use std::io;
use strsim::jaro_winkler;

pub async fn get_brand(name: String) -> Result<Vec<Brand>, Box<dyn std::error::Error>> {
    let collection_res = get_collection().await;
    if collection_res.is_err() {
        let err_msg: String = collection_res.err().unwrap().to_string();
        return Err(Box::new(io::Error::new(io::ErrorKind::Other, err_msg)));
    }
    let collection = collection_res.unwrap();

    // Regex to search db
    let regex = Regex {
        pattern: name.to_string(),
        options: "i".to_string(),
    };
    //

    let cursor_res = collection.find(doc! {"brand_name": regex}).await;

    if let Err(e) = cursor_res {
        return Err(Box::new(io::Error::new(io::ErrorKind::Other, e)));
    }

    let cursor = cursor_res.unwrap();

    let mut brands: Vec<Brand> = Vec::new();

    let results: Vec<_> = cursor.collect().await;
    // if cursor does not return data: check for text similarity
    if results.is_empty() {
        // get all brands names from db
        let cursor_result = collection.find(doc! {"_id": {"$ne": null}}).await;
        if cursor_result.is_err() {
            let err_msg: String = cursor_result.err().unwrap().to_string();
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, err_msg)));
        }
        let mut db_cursor = cursor_result.unwrap();

        while let Some(doc) = db_cursor.try_next().await.unwrap() {
            let similarity = jaro_winkler(
                &doc.brand_name.to_string().to_lowercase(),
                &name.to_string().to_lowercase(),
            );
            if similarity >= 0.8 {
                brands.push(doc);
            }
        }
    } else {
        // push brands from db
        for doc in results {
            brands.push(doc.unwrap());
        }
    }

    Ok(brands)
}

pub async fn add_brands(data: Vec<Brand>) -> Result<bool, Box<dyn std::error::Error>> {
    let collection_res = get_collection().await;
    if collection_res.is_err() {
        let err_msg: String = collection_res.err().unwrap().to_string();
        return Err(err_msg.into());
    }
    let collection = collection_res.unwrap();

    let cursor = collection.find(doc! {"_id": {"$ne": null}}).await;
    if cursor.is_err() {
        let err_msg: String = cursor.err().unwrap().to_string();
        return Err(err_msg.into());
    }
    let mut brands_cursor = cursor.unwrap();
    let mut brands: Vec<Brand> = Vec::new();

    while let Some(doc) = brands_cursor.try_next().await? {
        brands.push(doc);
    }

    // check if db is not empty and if data from scraping contains data that db does not have
    if brands.is_empty() {
        let data_clone = data.clone();
        let insert_res = collection.insert_many(data_clone).await;
        if insert_res.is_err() {
            let err_msg: String = insert_res.err().unwrap().to_string();
            return Err(err_msg.into());
        }
    } else {
        let diff: Vec<_> = data
            .iter()
            .filter(|e| {
                !brands
                    .iter()
                    .any(|b| b.brand_name == e.brand_name && b.source == e.source)
            })
            .collect();
        if !diff.is_empty() {
            let insert_res = collection.insert_many(diff).await;
            if insert_res.is_err() {
                let err_msg: String = insert_res.err().unwrap().to_string();
                return Err(err_msg.into());
            }
        }
    }

    Ok(true)
}

async fn get_collection() -> Result<Collection<Brand>, Box<dyn std::error::Error>> {
    let client_options = ClientOptions::parse(
        "mongodb+srv://rickyrickcastle:k7dACmdjy9rl2U7X@cluster0.ytfx6.mongodb.net/",
    )
    .await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("boycottdata");
    let brands_collection: Collection<Brand> = db.collection("brands");

    Ok(brands_collection)
}
