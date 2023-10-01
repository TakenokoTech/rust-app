use std::collections::HashMap;
use std::mem::replace;
use std::sync::Arc;
use serde_json::{from_str, to_string_pretty};
use tokio::time::*;
use tokio::sync::Mutex;
use tokio::fs::{File, read_to_string};
use tokio::io::AsyncWriteExt;
use tokio::time::Duration;
use crate::*;
use crate::utils::*;

pub struct DataRepository {
    data: SharedData,
    lock: SharedLock,
}

impl DataRepository {
    const DATA_JSON_FILE: &'static str = "temp/data.json";

    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            data: new_shared_map!(),
            lock: new_shared_bool!(false),
        })
    }

    pub async fn load_all(&self) -> HashMap<String, String> {
        let contents = match read_to_string(Self::DATA_JSON_FILE).await {
            Ok(size) => size,
            Err(err) => return error_handler!(err, HashMap::new())
        };
        let map = match from_str::<HashMap<String, String>>(&contents) {
            Ok(data) => data, //success_handler!(data),
            Err(err) => return error_handler!(err, HashMap::new())
        };
        map
    }

    pub async fn load(&self, key: String) -> HashMap<String, String> {
        let mut map = HashMap::new();
        async_let(&self.data, |data| {
            if let Some(value) = data.get(&*key) {
                map.insert(key.clone(), value.clone());
            }
        }).await;
        if let Some(value) = self.load_all().await.get(key.as_str()) {
            map.insert(key, value.clone());
        }
        return map;
    }

    pub async fn save(&self, key: String, value: String) {
        async_let(&self.data, |data| data.insert(key, value)).await;
        if !async_let(&self.lock, |it| !replace(it, true)).await {
            sleep(Duration::from_secs(1)).await;
        }
        let mut new_lock = self.lock.lock().await;
        let mut new_data = copy_let(&self.data).await;
        if !new_data.is_empty() {
            new_data.extend(self.load_all().await);
            self.write_data(new_data.clone()).await;
            async_let(&self.data, |data| {
                new_data.iter().for_each(|(key, _)| { data.remove(key); })
            }).await;
        }
        *new_lock = false;
    }

    pub async fn remove_all(&self) {
        async_let(&self.data, |data| data.clear()).await;
        self.write_data(HashMap::new()).await;
    }

    pub async fn remove(&self, key: String) {
        let new_lock = self.lock.lock().await;
        async_let(&self.data, |data| data.remove(&*key)).await;
        let map = self.load_all().await;
        let data = filter(map, |(k, _)| *k.clone() != key);
        self.write_data(data).await;
        drop(new_lock);
    }

    async fn write_data(&self, data: HashMap<String, String>) {
        // info!("write_data. len={:?}", data.len());
        // sleep(Duration::from_secs(1)).await;
        let sorted_map = sort_map(data);
        let json_data = to_string_pretty(&sorted_map.clone()).unwrap();
        let mut file = File::create(Self::DATA_JSON_FILE).await.unwrap();
        file.write_all(json_data.as_bytes()).await.unwrap();
    }
}
