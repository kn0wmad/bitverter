#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

use maud::{html, Markup};
use rocket::{get, http::RawStr, routes};

#[get("/")]
fn landing<'a>() -> Markup {
    html! {
        h1 { "bitverter" }
        p { "how many btc can i buy wit usd"}
        form action="/convert" method="get" {
            input name="amount" type="number" {}
            input type="submit" value="convert!!" {}
        }
    }
}

#[get("/convert?<amount>")]
fn conversion<'a>(amount: &RawStr) -> Markup {
    html! {
        h1 { "USD -> BTC" }
        p { (amount) " BTC" }
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![landing, conversion])
        .launch();
}
