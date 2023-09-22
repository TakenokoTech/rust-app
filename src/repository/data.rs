use std::collections::HashMap;
use std::sync::Arc;
use serde_json::*;
use tokio::time::*;
use tokio::sync::Mutex;
use tokio::fs::File;
use tokio::io::*;
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
        let file = File::open(Self::DATA_JSON_FILE).await.unwrap();
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        match reader.read_to_string(&mut contents).await {
            Ok(size) => size, //success_handler!(size),
            Err(err) => return error_handler!(err, HashMap::new())
        };
        return match from_str::<HashMap<String, String>>(&contents) {
            Ok(data) => success_handler!(data),
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
        async_let(self.data.clone(), |mut data| data.insert(key, value)).await;
        if !self.trylock().await {
            sleep(Duration::from_secs(5)).await;
        }
        self.flash().await;
        self.unlock().await;
    }

    pub async fn remove_all(&self) {
        async_let(self.data.clone(), |mut data| data.clear()).await;
        self.write_data(HashMap::new()).await;
    }

    pub async fn remove(&self, key: String) {
        let map = self.load_all().await;
        let data = filter(map, |(k, _)| *k.clone() != key);
        self.write_data(data).await;
        sleep(Duration::from_secs(1)).await;
    }

    async fn trylock(&self) -> bool {
        let mut write_pending = self.lock.lock().await;
        let can_lock = !*write_pending;
        *write_pending = true;
        drop(write_pending);
        return can_lock;
    }

    async fn unlock(&self) {
        let mut write_pending = self.lock.lock().await;
        *write_pending = false;
        drop(write_pending);
    }

    async fn flash(&self) {
        if let Ok(mut data) = self.data.try_lock() {
            if !data.is_empty() {
                data.extend(self.load_all().await);
                self.write_data(data.clone()).await;
                data.clear();
            }
            drop(data);
        }
    }

    async fn write_data(&self, data: HashMap<String, String>) {
        sleep(Duration::from_secs(1)).await;
        let json_data = to_string_pretty(&data.clone()).unwrap();
        let mut file = File::create(Self::DATA_JSON_FILE).await.unwrap();
        file.write_all(json_data.as_bytes()).await.unwrap();
    }
}
