use sycamore::prelude::*;
use sycamore::component::Component;
use log::info;
use web_sys::{Event, HtmlElement};
use wasm_bindgen::{prelude::*};
use js_sys::Array;

#[derive(Debug, Clone)]
pub struct BaseState {
    // pub page_wrapper: NodeRef<G>
}

#[component(BaseLayout<G>)]
pub fn base_layout<C: Component<G, Props = i32>>() -> Template<G> {
    // let base_state = BaseState {
    //     page_wrapper: NodeRef<G>::new()
    // };

    let wrapper_ref = NodeRef::new();

    let handle_click = cloned!((wrapper_ref) => move |event : Event| {
        info!("Sidebar toggle");

        let wrapper = wrapper_ref.get::<DomNode>().unchecked_into::<HtmlElement>();
        if wrapper.get_attribute("data-sidebar-hidden").is_some() {
            wrapper.remove_attribute("data-sidebar-hidden").expect("Failed to remove attribute");
        } else {
            wrapper.set_attribute("data-sidebar-hidden", "hidden").expect("Failed to set attribute.");
        }        

        event.stop_propagation();
    });

    template! {
        div(ref=wrapper_ref, class="page-wrapper with-navbar with-sidebar with-navbar-fixed-bottom") {
            div(class="sticky-alerts")
            nav(class="navbar") {
                button(class="btn btn-sm", type="button", on:click=handle_click){ "Sidebar" }
                button(class="btn btn-sm", type="button", on:click=toggle_dark_mode){ "Dark Mode" }
            }
            div(class="sidebar-overlay")
            div(class="sidebar")
            div(class="content-wrapper") {
                C(0)
            }
            nav(class="navbar navbar-fixed-bottom")
        }
    }
}

fn toggle_dark_mode(e: Event){
    // log("dark toggle");
    info!("Dark toggle");
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
	let body = document.body().expect("document expect to have have a body");
    let tokens = body.class_list();
    if tokens.contains("dark-mode") {
        body.class_list().remove(&to_array(&["dark-mode"])).expect("Failed to remove dark mode");
    } else {
        tokens.add(&to_array(&["dark-mode"])).expect("Failed to add dark mode ");
        body.class_list().set_value(&tokens.value());
    }
    
    e.stop_propagation()
}

pub fn to_array(strings: &[&str] ) -> Array {

    let arr = Array::new_with_length(strings.len() as u32);
    for (i, s) in strings.iter().enumerate() {
        arr.set(i as u32, JsValue::from_str(s));
    }
    arr
}




