use std::sync::Arc;
use tokio::sync::Mutex;

pub fn scope_let<T, F, R>(
    obj: T,
    f: F,
) -> R where F: FnOnce(T) -> R {
    f(obj)
}

pub async fn async_let<T, F, R>(
    obj: &Arc<Mutex<T>>,
    f: F,
) -> R where F: FnOnce(&mut T) -> R {
    let mut guard = obj.lock().await;
    scope_let(&mut *guard, f)
}

// pub fn scope_apply<T, F>(
//     mut obj: T,
//     f: F,
// ) -> T where F: FnOnce(&mut T) {
//     f(&mut obj);
//     obj
// }


// pub async fn async_also<T, F, R>(
//     obj: &Arc<Mutex<T>>,
//     f: F,
// ) -> R where F: FnOnce(&mut T) -> T {
//     let mut guard = obj.lock().await;
//     scope_let(&mut *guard, f)
// }
