mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{window,Document,Element};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn prompt(s: &str)->String;
}

#[wasm_bindgen]
pub fn greet() {
    let mut inputStr = prompt("HELLO WORLD");
    append_h1_to_body(&inputStr);
    alert(&inputStr);
}

#[wasm_bindgen]
pub fn append_h1_to_body(s:&str) -> String{
    let window = window().unwrap();
    let document = window.document().expect("document not found");
    let h1 = document.create_element("h1").expect("create h1 failed");
    h1.set_text_content(Some(s));
    let body = document.body().expect("body not found");
    body.append_child(&h1).expect("failed to append h1");
    
    return s.to_string();
}

#[wasm_bindgen]
pub fn init(){
    
}