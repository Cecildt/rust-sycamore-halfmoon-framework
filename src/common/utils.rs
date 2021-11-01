use js_sys::Array;
use log::info;
use wasm_bindgen::prelude::*;

pub fn to_array(strings: &[&str]) -> Array {
    let arr = Array::new_with_length(strings.len() as u32);
    for (i, s) in strings.iter().enumerate() {
        arr.set(i as u32, JsValue::from_str(s));
    }
    arr
}

#[allow(unused_variables, dead_code)]
pub fn random_id(length: i32) -> String {
    let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string();
    let chars_length = characters.len() as f64;
    let mut result = "".to_string();

    for x in 1..length {
        let pos = js_sys::Math::floor(js_sys::Math::random() * chars_length) as i32;
        result.push(characters.chars().nth(pos as usize).to_owned().expect("Invalid index value"));
    }

    info!("Random id generated: {}", &result);
    result
}