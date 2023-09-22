use std::collections::HashMap;
use std::sync::Arc;
use tokio::spawn;
use crate::repository::DataRepository;
use crate::utils::*;

pub struct Usecase {
    repo: Arc<DataRepository>,
}

impl Usecase {
    pub fn new(repo: Arc<DataRepository>) -> Arc<Self> {
        Arc::new(Self { repo })
    }

    pub async fn get(
        &self,
        authorization: String,
        query: HashMap<String, String>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        log::info!("Usecase.get: {:?} {:?}", authorization, query);
        let result = match query.get("q") {
            Some(value) => self.repo.load(value.clone()).await,
            None => self.repo.load_all().await
        };
        let sorted_map = sort_map(result);
        return Ok(warp::reply::json(&sorted_map));
    }

    pub async fn post(
        &self,
        authorization: String,
        data: HashMap<String, String>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        log::info!("Usecase.post: {:?} {:?}", authorization, data);
        for (key, value) in data {
            let repo = self.repo.clone();
            spawn(async move { repo.save(key, value).await });
        }
        return Ok(warp::reply::json(&"success"));
    }

    pub async fn delete(
        &self,
        authorization: String,
        query: HashMap<String, String>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        log::info!("Usecase.delete: {:?} {:?}", authorization, query);
        match query.get("q") {
            Some(value) => self.repo.remove(value.clone()).await,
            None => self.repo.remove_all().await
        };
        return Ok(warp::reply::json(&"success"));
    }
}
