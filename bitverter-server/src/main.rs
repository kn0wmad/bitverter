use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use maud::html;
use serde::Deserialize;
use tokio::fs;

async fn index() -> impl Responder {
    let body = html! {
        h1 { "bitverter" }
        p { "how many btc can i buy wit usd"}
        form action="/convert" method="get" {
            input name="amount" type="number" {}
            input type="submit" value="convert!!" {}
        }
    };

    HttpResponse::Ok().body(body.into_string())
}

#[derive(Deserialize)]
struct ConvertParams {
    amount: String,
}

#[derive(Deserialize)]
struct Config {
    nomics_api_key: String,
}

#[derive(Deserialize)]
struct NomicsPriceResponse {
    price: String,
}

async fn convert(params: web::Query<ConvertParams>) -> impl Responder {
    let config_str = fs::read("config.json").await.unwrap();
    let config: Config = serde_json::from_slice(&config_str).unwrap();

    let btc_usd_price_request_url = format!(
        "https://api.nomics.com/v1/currencies/ticker?key={}&ids=BTC&interval=1d&convert=USD",
        config.nomics_api_key
    );

    let response = reqwest::get(&btc_usd_price_request_url).await.unwrap();
    let btc_usd_price: Vec<NomicsPriceResponse> = response.json().await.unwrap();

    let price = btc_usd_price[0].price.parse::<f64>().unwrap();
    let amount = params.amount.parse::<f64>().unwrap();

    let result = amount / price;

    let body = html! {
        h1 { "USD -> BTC" }
        p { (result) " BTC" }
    };

    HttpResponse::Ok().body(body.into_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/convert", web::get().to(convert))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
