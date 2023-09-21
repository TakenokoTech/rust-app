mod routes;
mod repository;
mod utils;
mod usecase;

#[tokio::main]
async fn main() {
    utils::init_logger();
    let repo = repository::DataRepository::new();
    let usecase = usecase::Usecase::new(repo);
    let routes = routes::data_routes(usecase);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
