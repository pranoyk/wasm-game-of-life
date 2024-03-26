mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let msg = format!("Hello, {}! Welcome to the world of WebAssembly", name);
    alert(&msg);
}
