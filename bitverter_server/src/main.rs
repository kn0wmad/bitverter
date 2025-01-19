use std::error::Error;
use std::net::SocketAddr;

use axum::{routing::get, routing::get_service, Router};
use tower_http::services::ServeDir;

async fn test() -> &'static str {
    "Welcome to Bitverter!"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // build application with a single route
    let app = Router::new()
        .fallback_service(get_service(ServeDir::new("./site")))
        .route("/api/test", get(test).post(test));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())

    // // run app with hyper, listening globally on port 3333
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}
