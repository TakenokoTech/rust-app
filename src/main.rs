#![allow(unused_imports)]
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /data route
    let get_data = warp::path("data")
        .and(warp::get())
        .map(|| "GET /data endpoint");

    // Combine all routes
    let routes = get_data;

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

