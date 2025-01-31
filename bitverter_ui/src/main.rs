#![allow(non_snake_case)]

use dioxus::prelude::*;

// mod components;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        h1 { "Bitverter" }
        span { "Welome to Bitverter!" }

    // Global app resources
    // document::Link { rel: "icon", href: FAVICON }
    // document::Link { rel: "stylesheet", href: MAIN_CSS }
    }
}
