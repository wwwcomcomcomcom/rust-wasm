mod utils;

extern crate console_error_panic_hook;
use std::panic;

use wasm_bindgen::prelude::*;
use web_sys::window;
// use chrono::prelude::*;
// let date_as_string = Utc::now().to_string();

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn prompt(s: &str)->String;
    
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn clearInterval(token: f64);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Interval {
    closure: Closure<dyn FnMut()>,
    token: f64,
}

impl Interval {
    pub fn new<F: 'static>(millis: u32, f: F) -> Interval
    where
        F: FnMut()
    {
        // Construct a new closure.
        let closure = Closure::new(f);

        // Pass the closure to JS, to run every n milliseconds.
        let token = setInterval(&closure, millis);

        Interval { closure, token }
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        clearInterval(self.token);
    }
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
#[wasm_bindgen]
pub fn hello() -> Interval {
    Interval::new(1000, || log("hello"))
}