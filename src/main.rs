use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use maud::html;
use serde::Deserialize;

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
struct ConversionParmams {
    amount: String,
}

async fn conversion(params: web::Query<ConversionParmams>) -> impl Responder {
    let body = html! {
        h1 { "USD -> BTC" }
        p { (params.amount) " BTC" }
    };

    HttpResponse::Ok().body(body.into_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/convert", web::get().to(conversion))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
