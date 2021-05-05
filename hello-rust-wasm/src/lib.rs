use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}", name));
}

#[wasm_bindgen(start)]
pub fn run() {
    bare_bones();
    using_a_macro();
    using_web_sys();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    // Note that we need to use `js_name` to ensure we always call `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

fn bare_bones() {
    log("Hello from Rust!");
    log_u32(42);
    log_many("Logging", "many values!");
}

macro_rules! console_log {
    // using the `log` function imported above during `bare_bones`.
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()));
}
fn using_a_macro() {
    console_log!("Hello {}!", "world");
    console_log!("Let's print some numbers...");
    console_log!("1 + 3 = {}", 1 + 3);
}

fn using_web_sys() {
    // using an API requires enabling the features for all types used in the API.
    use web_sys::console::{log_1, log_2};
    log_1(&"Hello using web-sys".into());
    let js: JsValue = 4.into();
    log_2(&"Logging arbitrary values looks like".into(), &js);
}
