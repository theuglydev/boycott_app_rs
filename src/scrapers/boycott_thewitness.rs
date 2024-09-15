use async_std;
use chromiumoxide::{handler::viewport::Viewport, Browser, BrowserConfig, Page};
use futures::StreamExt;

pub async fn init() -> Result<usize, Box<dyn std::error::Error>> {
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .request_timeout(std::time::Duration::from_secs(10000000))
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
        .new_page("https://boycott.thewitness.news/browse/1")
        .await?;
    make_delay(2).await;

    let numbers_res = collect_page_numbers(&page).await;
    if numbers_res.is_err() {
        let err_msg: String = numbers_res.err().unwrap().to_string();
        return Err(err_msg.into());
    }
    let page_numbers: usize = numbers_res.unwrap();
    println!("{}", page_numbers);

    // finish scraping
    browser.close().await?;
    handle.await;
    //

    Ok(page_numbers)
}

async fn collect_page_numbers(page: &Page) -> Result<usize, Box<dyn std::error::Error>> {
    let navigation_buttons = page.find_elements("button[class='mantine-focus-auto mantine-active m-326d024a mantine-Pagination-control m-87cf2631 mantine-UnstyledButton-root']").await?;

    let page_numbers = navigation_buttons.len();

    Ok(page_numbers)
}

async fn make_delay(dur: u64) {
    async_std::task::sleep(std::time::Duration::from_secs(dur)).await;
}
