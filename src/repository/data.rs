use std::collections::HashMap;
use std::sync::Arc;
use serde_json::to_string_pretty;
use tokio::time::sleep;
use tokio::sync::Mutex;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::time::Duration;
use crate::{new_shared_bool, new_shared_map};

pub struct DataRepository {
    data: Arc<Mutex<HashMap<String, String>>>,
    lock: Arc<Mutex<bool>>,
}

impl DataRepository {
    const DATA_JSON_FILE: &'static str = "temp/data.json";

    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            data: new_shared_map!(),
            lock: new_shared_bool!(false),
        })
    }

    pub async fn save(&self, key: String, value: String) {
        log::info!("data: {:?}", self.data);
        log::info!("lock: {:?}", self.lock);

        let mut data = self.data.lock().await;
        data.insert(key, value);
        drop(data);

        if !self.trylock().await { return; }

        let data = self.data.lock().await;
        let json_data = to_string_pretty(&data.clone()).unwrap();
        drop(data);

        let mut file = File::create(Self::DATA_JSON_FILE).await.unwrap();
        sleep(Duration::from_secs(1)).await;
        file.write_all(json_data.as_bytes()).await.unwrap();
        self.unlock().await;
    }

    pub async fn load(&self, key: String) -> Option<String> {
        let data = self.data.lock().await;
        data.get(&key).cloned()
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
}
