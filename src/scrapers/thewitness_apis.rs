use crate::models::brand::Brand;
use reqwest;
use serde_json::Value;

pub async fn get_data_from_thewitness_apis(
    page_numbers: usize,
) -> Result<Vec<Brand>, Box<dyn std::error::Error>> {
    let mut brands: Vec<Brand> = Vec::new();

    // api base urls
    let brands_base_url: String =
        String::from("https://boycott.thewitness.news/_next/data/uvfGe2UMf0aTAwzzmwmql/browse/");
    let proof_base_url: String =
        String::from("https://boycott.thewitness.news/_next/data/uvfGe2UMf0aTAwzzmwmql/target/");
    //

    for i in 1..page_numbers {
        // brands url dynamic part
        let brands_dyn_part: String = i.to_string() + &String::from(".json?page=") + &i.to_string();
        let brands_final_url: String = String::from(&brands_base_url) + &brands_dyn_part;

        let brands_res = reqwest::get(brands_final_url).await;
        if brands_res.is_err() {
            let err_msg: String = brands_res.err().unwrap().to_string();
            return Err(err_msg.into());
        }

        let brands_unwrapped = brands_res.unwrap();
        if brands_unwrapped.status() != 200 {
            let err_msg: String = brands_unwrapped.status().to_string() + &String::from(" Error");
            return Err(err_msg.into());
        }

        let brands_text = brands_unwrapped.text().await?;
        let brands_value: Value = serde_json::from_str(&brands_text)?;

        // get the listings array from the json to get to the proof for each brand
        if let Some(page_props) = brands_value.get("pageProps") {
            if let Some(listings) = page_props.get("listings").and_then(|v| v.as_array()) {
                for brand in listings {
                    let brand_id: String = brand.get("id").unwrap().to_string();

                    // create the proof final url
                    let proof_final_url: String = String::from(&proof_base_url)
                        + &brand_id.trim_matches('"')
                        + &String::from(".json?id=")
                        + &brand_id.trim_matches('"');
                    //

                    let proof_res = reqwest::get(proof_final_url).await;
                    if proof_res.is_err() {
                        let err_msg: String = String::from("Error in retreiving proof ")
                            + &proof_res.err().unwrap().to_string();
                        return Err(err_msg.into());
                    }

                    let proof_unwrapped = proof_res.unwrap();
                    if proof_unwrapped.status() != 200 {
                        let err_msg: String = proof_unwrapped.status().to_string()
                            + &String::from(" Error in retrieving proof");
                        return Err(err_msg.into());
                    }

                    let proof_json = proof_unwrapped.text().await?;
                    let proof_value: Value = serde_json::from_str(&proof_json)?;

                    // extract IMPORTANT data
                    if let Some(props) = proof_value.get("pageProps") {
                        if let Some(listing) = props.get("listing") {
                            let brand_image: String = String::from(
                                listing
                                    .get("logo")
                                    .unwrap()
                                    .to_string()
                                    .replace("\\", "")
                                    .trim_matches('"'),
                            );
                            let brand_name: String = String::from(
                                listing
                                    .get("name")
                                    .unwrap()
                                    .to_string()
                                    .replace("\\", "")
                                    .trim_matches('"'),
                            );
                            let proof: String = String::from(
                                String::from(
                                    listing
                                        .get("description")
                                        .unwrap()
                                        .to_string()
                                        .replace("\\", "")
                                        .trim_matches('"'),
                                ) + &String::from("\n\n")
                                    + &listing
                                        .get("reason")
                                        .unwrap()
                                        .to_string()
                                        .replace("\\", "")
                                        .trim_matches('"'),
                            );
                            let source: String = String::from("boycott.thewitness.news");

                            // create new brand instance
                            let new_brand: Brand = Brand {
                                brand_name,
                                brand_image,
                                proof,
                                source,
                            };
                            brands.push(new_brand);
                        }
                    }
                }
            }
        }
    }

    Ok(brands)
}
