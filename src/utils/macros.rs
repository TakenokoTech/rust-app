#[macro_export]
macro_rules! new_shared_map {() => { Arc::new(Mutex::new(HashMap::new())) }}

#[macro_export]
macro_rules! new_shared_bool {($b: expr) => { Arc::new(Mutex::new($b)) }}
