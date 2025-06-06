//! Web server
use std::error::Error;
use std::net::SocketAddr;
use axum::{routing::{get, get_service, post}, Form, Router};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct Conversion {
    quantity: f64,
    base_unit: String,
    quote_unit: String,
}

/// Test API
async fn hander_test_route() -> &'static str {
    "Welcome to Bitverter! \nThis is the testing API"
}

/// Conversion function using bitverter_lib
/// Take user input from web form
async fn convert(form: Form<Conversion>) -> String {
    let result = bitverter_lib::convert(form.quantity, &form.base_unit, &form.quote_unit);
    format!("{}", result)
}

pub async fn run_server() -> Result<(), Box<dyn Error>> {
    // build application with a single route
    let router = Router::new()
        .fallback_service(get_service(ServeDir::new("./site")))
        .route("/api/convert", post(convert))
        .route("/api/test", get(hander_test_route));

    // Listen on localhost
    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    println!("Listening on {}", addr);

    // Start server and bind to above address
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   run_server().await
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn should_return_message_when_calling_test_handler() {
        assert!(hander_test_route().await == "Welcome to Bitverter! \nThis is the testing API");
    }

    #[tokio::test]
    async fn should_return_ok_when_calling_test_route() {
        reqwest::get("http://localhost:3333/api/test").await.unwrap().text().await.unwrap();
    }

    #[tokio::test]
    async fn should_return_100_000_000_sats_when_given_1_btc() {
        let client = reqwest::Client::new();

        let form_data = [
            ("quantity", "1"),
            ("base_unit", "btc"),
            ("quote_unit", "sats"),
        ];

        let res = client.post("http://localhost:3333/api/convert")
             .form(&form_data).send().await.unwrap();

        let conversion_amount = res.text().await.unwrap();
        assert_eq!(conversion_amount, 100_000_000.to_string());
    }

    #[tokio::test]
    async fn should_return_1_btc_when_given_100_000_000_sats() {
        let client = reqwest::Client::new();

        let form_data = [
            ("quantity", "100000000"),
            ("base_unit", "sats"),
            ("quote_unit", "btc"),
        ];

        let res = client.post("http://localhost:3333/api/convert")
             .form(&form_data).send().await.unwrap();

        let conversion_amount = res.text().await.unwrap();
        assert_eq!(conversion_amount, 1.to_string());
    }
}
