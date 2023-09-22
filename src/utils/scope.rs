use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

pub fn scope_let<T, F, R>(
    obj: T,
    f: F,
) -> R where F: FnOnce(T) -> R {
    f(obj)
}

pub async fn async_let<T, F, R>(
    obj: Arc<Mutex<T>>,
    f: F,
) -> R where F: FnOnce(MutexGuard<T>) -> R {
    scope_let(obj.lock().await, f)
}

// pub fn scope_apply<T, F>(
//     mut obj: T,
//     f: F,
// ) -> T where F: FnOnce(&mut T) {
//     f(&mut obj);
//     obj
// }
