mod db;
mod handlers;
mod models;
mod scrapers;
mod telegram;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route(
            "/scrapebrands",
            web::get().to(handlers::brands_handlers::scrape_brands_handler),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

    // let bot_task = tokio::task::spawn(async {
    //     telegram::bot::init_bot().await;
    // });

    // let server_task = HttpServer::new(|| {
    //     App::new().route(
    //         "/scrapebrands",
    //         web::get().to(handlers::brands_handlers::scrape_brands_handler),
    //     )
    // })
    // .bind("127.0.0.1:8080")?
    // .run();

    // // Wait for both tasks concurrently
    // tokio::select! {
    //     result = bot_task => {
    //         // Handle bot task error (if any)
    //         if let Err(err) = result {
    //             eprintln!("Bot task failed: {:?}", err);
    //         }
    //     }
    //     result = server_task => {
    //         // Handle the server task result
    //         if let Err(err) = result {
    //             return Err(err);
    //         }
    //     }
    // }

    // Ok(())
}

// #[tokio::main]
// async fn main() {
// }
