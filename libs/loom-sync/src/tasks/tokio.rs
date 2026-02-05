/// Spawn a task for async or blocking work.
///
/// # Patterns
/// - `spawn!()` - create task/resolver pair only
/// - `spawn!(|| expr)` - blocking closure on thread pool
/// - `spawn!(move || expr)` - blocking closure with move
/// - `spawn!(async { ... })` - async block
/// - `spawn!(future)` - any future
/// - `spawn!(|| expr, result)` - blocking closure returning Result
/// - `spawn!(async { ... }, result)` - async block returning Result
///
/// # Examples
/// ```ignore
/// let (task, resolver) = spawn!();
/// let task = spawn!(|| expensive_computation());
/// let task = spawn!(async { fetch().await });
/// let task = spawn!(async { try_fetch().await }, result);
/// ```
#[macro_export]
macro_rules! spawn {
    // Create task/resolver pair only
    () => {{
        let (sender, receiver) = $crate::open!(1);
        let task = $crate::tasks::Task::new(receiver);
        let handle = $crate::tasks::TaskResolver::new(task.id(), sender);
        (task, handle)
    }};

    // Blocking closure: spawn!(|| { ... })
    (|| $body:expr) => {{
        let (task, handle) = $crate::spawn!();
        $crate::internal::tokio::task::spawn_blocking(move || {
            let result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| $body));
            match result {
                Ok(value) => {
                    let _ = handle.ok(value);
                }
                Err(panic_info) => {
                    let msg = $crate::tasks::tokio::panic_payload_to_string(panic_info);
                    let _ = handle.fail($crate::tasks::TaskError::panic(msg));
                }
            }
        });
        task
    }};

    // Blocking closure with move: spawn!(move || { ... })
    (move || $body:expr) => {{
        let (task, handle) = $crate::spawn!();
        $crate::internal::tokio::task::spawn_blocking(move || {
            let result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || $body));
            match result {
                Ok(value) => {
                    let _ = handle.ok(value);
                }
                Err(panic_info) => {
                    let msg = $crate::tasks::tokio::panic_payload_to_string(panic_info);
                    let _ = handle.fail($crate::tasks::TaskError::panic(msg));
                }
            }
        });
        task
    }};

    // Async block/future: spawn!(async { ... }) or spawn!(some_future)
    ($future:expr) => {{
        let (task, handle) = $crate::spawn!();
        $crate::internal::tokio::spawn(async move {
            let result = $crate::internal::futures::FutureExt::catch_unwind(
                ::std::panic::AssertUnwindSafe($future),
            )
            .await;
            match result {
                Ok(value) => {
                    let _ = handle.ok_async(value).await;
                }
                Err(panic_info) => {
                    let msg = $crate::tasks::tokio::panic_payload_to_string(panic_info);
                    let _ = handle
                        .fail_async($crate::tasks::TaskError::panic(msg))
                        .await;
                }
            }
        });
        task
    }};

    // Blocking closure returning Result: spawn!(|| { ... }, result)
    (|| $body:expr, result) => {{
        let (task, handle) = $crate::spawn!();
        $crate::internal::tokio::task::spawn_blocking(move || {
            let result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| $body));
            match result {
                Ok(Ok(value)) => {
                    let _ = handle.ok(value);
                }
                Ok(Err(e)) => {
                    let _ = handle.fail($crate::tasks::TaskError::custom(e));
                }
                Err(panic_info) => {
                    let msg = $crate::tasks::tokio::panic_payload_to_string(panic_info);
                    let _ = handle.fail($crate::tasks::TaskError::panic(msg));
                }
            }
        });
        task
    }};

    // Blocking closure with move returning Result: spawn!(move || { ... }, result)
    (move || $body:expr, result) => {{
        let (task, handle) = $crate::spawn!();
        $crate::internal::tokio::task::spawn_blocking(move || {
            let result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || $body));
            match result {
                Ok(Ok(value)) => {
                    let _ = handle.ok(value);
                }
                Ok(Err(e)) => {
                    let _ = handle.fail($crate::tasks::TaskError::custom(e));
                }
                Err(panic_info) => {
                    let msg = $crate::tasks::tokio::panic_payload_to_string(panic_info);
                    let _ = handle.fail($crate::tasks::TaskError::panic(msg));
                }
            }
        });
        task
    }};

    // Async returning Result: spawn!(async { ... }, result)
    ($future:expr, result) => {{
        let (task, handle) = $crate::spawn!();
        $crate::internal::tokio::spawn(async move {
            let result = $crate::internal::futures::FutureExt::catch_unwind(
                ::std::panic::AssertUnwindSafe($future),
            )
            .await;
            match result {
                Ok(Ok(value)) => {
                    let _ = handle.ok_async(value).await;
                }
                Ok(Err(e)) => {
                    let _ = handle.fail_async($crate::tasks::TaskError::custom(e)).await;
                }
                Err(panic_info) => {
                    let msg = $crate::tasks::tokio::panic_payload_to_string(panic_info);
                    let _ = handle
                        .fail_async($crate::tasks::TaskError::panic(msg))
                        .await;
                }
            }
        });
        task
    }};
}

/// Convert panic payload to a string message
pub fn panic_payload_to_string(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "unknown panic".to_string()
    }
}
