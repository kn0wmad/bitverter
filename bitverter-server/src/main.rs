use warp::Filter;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./index.html"));

    let site = warp::path("bitverter-server").and(warp::fs::dir("./bitverter-server/"));

    let routes = index.or(site);

    info!("Serving...");
    
    // Serve site on 127.0.0.1:3030
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}