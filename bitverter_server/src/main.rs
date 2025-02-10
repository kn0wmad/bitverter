use std::error::Error;
use std::net::SocketAddr;

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use tower_http::services::ServeDir;

#[derive(Debug)]
struct BvParams {
    denomination: Option<String>,
}

// Handler for Input
async fn handler_bv_input(Query(params): Query<BvParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_bv_input - {params:?}", "HANDLER");

    let denomination = params.denomination.as_deref().unwrap_or("Bitcoin");
    Html(format!("Denomination: {denomination}"))
}

// Test API
async fn test() -> &'static str {
    "Welcome to Bitverter! \nThis is the testing API"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // build application with a single route
    let routes_bitverter = Router::new()
        .fallback_service(get_service(ServeDir::new("./site")))
        .route("/api/test", get(test).post(test));

    // Listen on localhost
    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    println!("Listening on {}", addr);

    // Start server and bind to above address
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, routes_bitverter).await?;
    Ok(())
}
