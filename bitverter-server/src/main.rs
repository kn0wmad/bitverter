use warp::Filter;
use maud::{ DOCTYPE, html, Markup, Render };
extern crate pretty_env_logger;
#[macro_use] extern crate log;

pub struct Stylesheet(&'static str);

impl Render for Stylesheet {
    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" type="text/css" href=(self.0);
        }
    }
}

// const API_URL: &str = "https://api.nomics.com/v1/currencies/ticker?key={}&ids=BTC&interval=1d&convert=USD";

#[tokio::main]
async fn main() {

    pretty_env_logger::init();

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./index.html"));

    let site = 
        warp::path("bitverter-server")
        .and(warp::fs::dir("./bitverter-server/"));

    let routes = index.or(site);

    info!("Serving...");
    
    // Serve site on 127.0.0.1:3030
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1";
        title { (page_title) }
    }
}

pub fn page(title: &str) -> Markup {
    html! {
        // Add the header markup to the page
        (header(title))
        h1 { (title) }
    }
}