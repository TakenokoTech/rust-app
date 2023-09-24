use std::future::Future;
use std::sync::{Arc};
use tokio::sync::{Mutex, MutexGuard};

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

pub async fn await_let<T, F, Ft, R>(
    obj: &Arc<Mutex<T>>,
    f: F,
) -> R where
    for<'a> F: FnOnce(&'a mut T) -> Ft + 'a,
    Ft: Future<Output=R> {
    let mut guard = obj.lock().await;
    f(&mut *guard).await
}

pub async fn copy_let<T>(obj: &Arc<Mutex<T>>) -> T where T: Clone {
    obj.lock().await.clone()
}
