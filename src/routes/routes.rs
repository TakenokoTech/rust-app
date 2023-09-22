use warp::Filter;
use std::sync::Arc;
use std::collections::HashMap;

use crate::usecase::Usecase;

pub fn data_routes(usecase: Arc<Usecase>) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    let get_usecase = usecase.clone();
    let post_usecase = usecase.clone();
    let delete_usecase = usecase.clone();

    let get = warp::path("data")
        .and(warp::get())
        .and(warp::header("authorization"))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(move |auth: String, query: HashMap<String, String>| {
            let usecase = get_usecase.clone();
            async move { usecase.get(auth, query).await }
        });

    let post = warp::path("data")
        .and(warp::post())
        .and(warp::header("authorization"))
        .and(warp::body::json())
        .and_then(move |auth: String, body: HashMap<String, String>| {
            let usecase = post_usecase.clone();
            async move { usecase.post(auth, body).await }
        });

    let put = warp::path("data")
        .and(warp::put())
        .map(|| "PUT /data endpoint");

    let delete = warp::path("data")
        .and(warp::delete())
        .and(warp::header("authorization"))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(move |auth: String, query: HashMap<String, String>| {
            let usecase = delete_usecase.clone();
            async move { usecase.delete(auth, query).await }
        });

    get.or(post).or(put).or(delete)
}
