mod utils;

extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use std::panic;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-bsgs!");
}

#[wasm_bindgen]
pub fn init() {
    log("[Console Error Panic Hook] Initialised!");
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}