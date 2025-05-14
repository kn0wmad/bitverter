//! Web server
use std::error::Error;
use std::net::SocketAddr;
use axum::{routing::{get, get_service, post}, Form, Router};
use axum_macros::debug_handler;
use serde::Deserialize;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct Conversion {
    quantity: f64,
    base_unit: String,
    quote_unit: String,
}

/// Test API
async fn test() -> &'static str {
    "Welcome to Bitverter! \nThis is the testing API"
}

/// Conversion function using bitverter_lib
/// Take user input from web form
#[debug_handler]
async fn convert(form: Form<Conversion>) -> String {
    let result = bitverter_lib::convert(form.quantity, &form.base_unit, &form.quote_unit);
    format!("{}", result)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // build application with a single route
    let router = Router::new()
        .fallback_service(get_service(ServeDir::new("./site")))
        .route("/api/convert", post(convert))
        .route("/api/test", get(test));

    // Listen on localhost
    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    println!("Listening on {}", addr);

    // Start server and bind to above address
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
