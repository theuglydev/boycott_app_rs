use crate::models::brand::Brand;
use async_std;
use chromiumoxide::{
    cdp::js_protocol::runtime::EvaluateParams, handler::viewport::Viewport, Browser, BrowserConfig,
    Page,
};
use futures::StreamExt;

pub async fn init() -> Result<Vec<Brand>, Box<dyn std::error::Error>> {
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .viewport(Viewport {
                width: 0,
                height: 0,
                has_touch: false,
                is_landscape: false,
                emulating_mobile: false,
                device_scale_factor: None,
            })
            .with_head()
            .build()?,
    )
    .await?;

    let handle = async_std::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let page: Page = browser
        .new_page("https://www.ethicalconsumer.org/ethicalcampaigns/boycotts")
        .await?;
    make_delay(2).await;

    // accept cookies
    let cookies_button = page.find_element("button.cookie-compliance__button").await;
    if cookies_button.is_ok() {
        let button = cookies_button.unwrap();
        let click_res = button.click().await;
        if click_res.is_err() {
            // let err_msg: String = click_res.err().unwrap().to_string();
            // return Err(err_msg.into());
            println!("no cookies button");
        }

        make_delay(2).await;
    }
    //

    let scraping_res = scrape_data(&page).await;
    if scraping_res.is_err() {
        let err_msg: String = scraping_res.err().unwrap().to_string();
        return Err(err_msg.into());
    }
    let brands: Vec<Brand> = scraping_res.unwrap();
    println!("{:#?}", brands);
    println!("ethical consumer brands length: {}", brands.len());

    browser.close().await?;
    handle.await;

    Ok(brands)
}

async fn scrape_data(page: &Page) -> Result<Vec<Brand>, Box<dyn std::error::Error>> {
    let mut brands: Vec<Brand> = Vec::new();

    let divs = page.find_elements("div[class='tile boycott']").await;
    if divs.is_err() {
        let err_msg: String = divs.err().unwrap().to_string();
        return Err(err_msg.into());
    }
    let content_divs = divs.unwrap();

    for div in content_divs {
        // Brand Name
        let name_div = div.find_element("div.col-md-12>h3").await?;
        let brand_name: String = String::from(
            name_div
                .inner_text()
                .await?
                .unwrap()
                .to_string()
                .replace("\\", "")
                .trim_matches('"'),
        );
        println!("got name");

        // Brand Image
        let mut brand_image: String = String::new();
        let img_el = div.find_element("div>div.image>img").await;
        if img_el.is_ok() {
            brand_image = String::from(
                img_el
                    .unwrap()
                    .attribute("src")
                    .await?
                    .unwrap()
                    .to_string()
                    .replace("\\", "")
                    .trim_matches('"'),
            );
        } else {
            brand_image = String::from("https://st.depositphotos.com/2672167/3759/v/450/depositphotos_37595575-stock-illustration-the-man-silhouette.jpg");
        }
        println!("got image");

        // Proof
        let mut proof: String = String::new();
        let proof_el = div.find_element("div[class='field field--name-field-summary field--type-text-long field--label-hidden field--item']").await;
        if proof_el.is_ok() {
            let proof_unwrapped = proof_el.unwrap();
            if let Some(proof_text) = proof_unwrapped.inner_text().await? {
                proof = String::from(proof_text.to_string().replace("\\", "").trim_matches('"'));
            }
        }

        // Source
        let source: String = String::from("ethicalconsumer.org");

        // create an instance of Brand
        let new_brand: Brand = Brand {
            brand_name,
            brand_image,
            source,
            proof,
        };
        brands.push(new_brand);
    }

    Ok(brands)
}

async fn scroll_to_bottom(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
    let script = r#"
        window.scrollTo(0, 50)
        "#;
    for i in 0..15 {
        page.evaluate(EvaluateParams::builder().expression(script).build()?)
            .await?;

        make_delay(1).await;
    }

    Ok(())
}

async fn make_delay(dur: u64) {
    async_std::task::sleep(std::time::Duration::from_secs(dur)).await;
}
