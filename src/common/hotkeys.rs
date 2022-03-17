use log::info;
use wasm_bindgen::{JsCast, prelude::Closure};

#[allow(unused_must_use)]
pub fn init_hotkeys(){
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");

    let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        info!("key down: {}", event.target().unwrap().to_string());
    }) as Box<dyn FnMut(_)>);
    
    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
    closure.forget();

}