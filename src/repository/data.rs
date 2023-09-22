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
        return match from_str::<HashMap<String, String>>(&contents) {
            Ok(data) => data, //success_handler!(data),
            Err(err) => return error_handler!(err, HashMap::new())
        };
    }

    pub async fn load(&self, key: String) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if let Some(value) = self.load_all().await.get(key.as_str()) {
            map.insert(key, value.clone());
        }
        return map;
    }

    pub async fn save(&self, key: String, value: String) {
        async_let(&self.data, |data| data.insert(key, value)).await;
        if !async_let(&self.lock, |it| !replace(it, true)).await {
            sleep(Duration::from_secs(2)).await;
        }
        if let Ok(mut data) = self.data.try_lock() {
            if !data.is_empty() {
                data.extend(self.load_all().await);
                self.write_data(data.clone()).await;
                data.clear();
            }
            drop(data);
            async_let(&self.lock, |it| *it = false).await;
        }
    }

    pub async fn remove_all(&self) {
        async_let(&self.data, |data| data.clear()).await;
        self.write_data(HashMap::new()).await;
    }

    pub async fn remove(&self, key: String) {
        let map = self.load_all().await;
        let data = filter(map, |(k, _)| *k.clone() != key);
        self.write_data(data).await;
        sleep(Duration::from_secs(1)).await;
    }

    async fn write_data(&self, data: HashMap<String, String>) {
        log::info!("write_data. {:?}", data);
        sleep(Duration::from_secs(1)).await;
        let json_data = to_string_pretty(&data.clone()).unwrap();
        let mut file = File::create(Self::DATA_JSON_FILE).await.unwrap();
        file.write_all(json_data.as_bytes()).await.unwrap();
    }
}
