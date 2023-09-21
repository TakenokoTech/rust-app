use std::collections::HashMap;
use std::sync::Arc;
use crate::repository::DataRepository;

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
    ) -> Result<impl warp::Reply, warp::Rejection> {
        log::info!("Usecase.post: {:?}", authorization);
        let str = self.repo.load("key1".to_string()).await;
        return Ok(warp::reply::json(&str.unwrap()));
    }

    pub async fn post(
        &self,
        authorization: String,
        data: HashMap<String, String>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        log::info!("Usecase.post: {:?} {:?}", authorization, data);
        self.repo.save("key1".to_string(), "value1".to_string()).await;
        return Ok(warp::reply::json(&"success"));
    }
}
