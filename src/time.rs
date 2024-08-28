#[cfg(not(target_arch = "wasm32"))]
pub fn now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_secs_f64()
}

#[cfg(target_arch = "wasm32")]
pub fn now() -> f64 {
    let window = web_sys::window().unwrap();
    let performance = window.performance().unwrap();
    performance.now()
}
