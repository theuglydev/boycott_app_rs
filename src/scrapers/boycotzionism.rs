use crate::models::brand::Brand;
use async_std;
use chromiumoxide::{cdp::js_protocol::runtime::EvaluateParams, Browser, BrowserConfig, Page};
use futures::StreamExt;

pub async fn init() -> Result<Vec<Brand>, Box<dyn std::error::Error>> {
    let (mut browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build()?).await?;

    let handle = async_std::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let page: Page = browser.new_page("https://www.boycotzionism.com/").await?;
    make_delay(10).await;

    scroll_to_bottom(&page).await;

    let scraping_res = scrape_data(&page).await;
    if scraping_res.is_err() {
        let err_msg = scraping_res.err().unwrap().to_string();
        return Err(err_msg.into());
    }
    let brands = scraping_res.unwrap();
    println!("{:?}", brands.len());

    browser.close().await?;
    handle.await;

    Ok(brands)
}

async fn scrape_data(page: &Page) -> Result<Vec<Brand>, Box<dyn std::error::Error>> {
    // the final return array
    let mut brands: Vec<Brand> = Vec::new();
    //

    let cards_elements = page.find_elements("div.react-card-flip").await;
    if cards_elements.is_err() {
        let err_msg = cards_elements.err().unwrap().to_string();
        return Err(err_msg.into());
    }
    let cards = cards_elements.unwrap();

    for i in 0..cards.len() {
        let cards_to_clone = page.find_elements("div.react-card-flip").await?;
        let card = &cards_to_clone[i];

        let header = card.find_element("h1.text-ellipsis").await?;
        let brand_name = header.inner_text().await?.unwrap().to_string();

        let img = card.find_element("img[alt='brand logo']").await;
        let mut brand_image: String = String::new();
        if img.is_ok() {
            brand_image = img.unwrap().attribute("src").await?.unwrap().to_string();
        } else {
            brand_image = String::from("https://st.depositphotos.com/2672167/3759/v/450/depositphotos_37595575-stock-illustration-the-man-silhouette.jpg");
        }

        // get proof
        let button = card.find_element("button.bg-red-700").await?;
        button.click().await?;
        make_delay(1).await;

        let markdown_text = page.find_element("div.markdown").await?;
        let proof = markdown_text.inner_text().await?.unwrap().to_string();

        // exit proof dialog
        let exit_button = page.find_element("button[class='p-1 ml-auto bg-transparent border-0 text-black float-right leading-none outline-none focus:outline-none']").await?;
        exit_button.click().await?;

        let source: String = String::from("boycotzionism.com");

        make_delay(1).await;
        // DONE WITH PROOF

        let new_brand: Brand = Brand {
            brand_name,
            brand_image,
            proof,
            source,
        };
        brands.push(new_brand);
    }

    Ok(brands)
}

async fn scroll_to_bottom(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
    let mut end_p = page
        .find_element("p[class='text-center text-black text-lg my-5 font-semibold uppercase']")
        .await;

    while !end_p.is_ok() {
        let script = r#"
            window.scrollTo(0, document.body.scrollHeight);
            "#;

        let scroll_res = page
            .evaluate(EvaluateParams::builder().expression(script).build()?)
            .await;
        if scroll_res.is_err() {
            println!("{}", scroll_res.err().unwrap().to_string());
            println!("Could not scroll");
        }
        make_delay(1).await;

        end_p = page
            .find_element("p[class='text-center text-black text-lg my-5 font-semibold uppercase']")
            .await;
    }

    Ok(())
}

async fn make_delay(dur: u64) {
    async_std::task::sleep(std::time::Duration::from_secs(dur)).await;
}
