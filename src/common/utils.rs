use js_sys::Array;
use wasm_bindgen::prelude::*;

pub fn to_array(strings: &[&str]) -> Array {
    let arr = Array::new_with_length(strings.len() as u32);
    for (i, s) in strings.iter().enumerate() {
        arr.set(i as u32, JsValue::from_str(s));
    }
    arr
}