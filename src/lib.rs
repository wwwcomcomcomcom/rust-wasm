mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{window,Document,Element};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-wasm!");
}

#[wasm_bindgen]
pub fn append_h1_to_body() {
    let window = window().unwrap();
    let document = window.document().expect("document not found");
    let h1 = document.create_element("h1").expect("create h1 failed");
    h1.set_text_content(Some("Hello from Rust"));
    let body = document.body().expect("body not found");
    body.append_child(&h1).expect("failed to append h1");
}