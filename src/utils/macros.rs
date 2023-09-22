use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type SharedData = Arc<Mutex<HashMap<String, String>>>;
pub type SharedLock = Arc<Mutex<bool>>;

#[macro_export]
macro_rules! new_shared_map {() => { Arc::new(Mutex::new(HashMap::new())) }}

#[macro_export]
macro_rules! new_shared_bool {($b: expr) => { Arc::new(Mutex::new($b)) }}

#[macro_export]
macro_rules! success_handler {($data: expr) => {{ log::info!("success_handler: {:?}", $data); $data }}}

#[macro_export]
macro_rules! error_handler {($err: expr, $def: expr) => {{ log::error!("error_handler: {:?}", $err); $def }}}
