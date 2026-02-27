use std::{future::Future, sync::OnceLock};

use tokio::runtime::Handle;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

pub static RUNTIME: OnceLock<Runtime> = OnceLock::new();

pub fn default_runtime() -> Runtime {
    Runtime::new().unwrap()
}

pub fn get_runtime_handle() -> Handle {
    match Handle::try_current() {
        Ok(handle) => handle,
        Err(_) => {
            let runtime = RUNTIME.get_or_init(default_runtime);
            runtime.handle().clone()
        }
    }
}

/// Runs a future to completion on runtime.
pub fn block_on<F: Future>(task: F) -> F::Output {
    let handle = get_runtime_handle();
    handle.block_on(task)
}

pub fn spawn<F>(task: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let handle = get_runtime_handle();
    handle.spawn(task)
}
