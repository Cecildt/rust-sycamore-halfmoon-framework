use log::info;
use sycamore::component::Component;
use sycamore::prelude::*;
use web_sys::{Event, HtmlElement};

use crate::common::utils::to_array;

#[derive(Debug, Clone)]
pub struct BaseState {
    pub dark_mode: Signal<bool>,
    pub show_sidebar: Signal<bool>,
}

#[component(BaseLayout<G>)]
pub fn base_layout<C: Component<G, Props = i32>>() -> Template<G> {
    let base_state = BaseState {
        dark_mode: Signal::new(true),
        show_sidebar: Signal::new(true),
    };

    let wrapper_ref = NodeRef::new();

    let handle_sidebar_click = cloned!((wrapper_ref, base_state) => move |event : Event| {
        info!("Sidebar toggle");

        let wrapper = wrapper_ref.get::<DomNode>().unchecked_into::<HtmlElement>();
        if wrapper.get_attribute("data-sidebar-hidden").is_some() {
            wrapper.remove_attribute("data-sidebar-hidden").expect("Failed to remove attribute");
            base_state.show_sidebar.set(true);
        } else {
            wrapper.set_attribute("data-sidebar-hidden", "hidden").expect("Failed to set attribute.");
            base_state.show_sidebar.set(false);
        }

        event.stop_propagation();
    });

    let handle_darkmode_click = cloned!((base_state) => move |event : Event| {
        info!("Dark toggle");
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let body = document.body().expect("document expect to have have a body");
        let tokens = body.class_list();
        if tokens.contains("dark-mode") {
            body.class_list().remove(&to_array(&["dark-mode"])).expect("Failed to remove dark mode");
            base_state.dark_mode.set(false);
        } else {
            tokens.add(&to_array(&["dark-mode"])).expect("Failed to add dark mode ");
            body.class_list().set_value(&tokens.value());
            base_state.dark_mode.set(true);
        }

        event.stop_propagation()
    });

    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let client_width = document.document_element().unwrap().client_width();

    // Set dark mode
    let body = document
        .body()
        .expect("document expect to have have a body");
    let tokens = body.class_list();
    tokens
        .add(&to_array(&["dark-mode"]))
        .expect("Failed to add dark mode ");
    body.class_list().set_value(&tokens.value());

    if client_width <= 768 {
        template! {
            div(ref=wrapper_ref, class="page-wrapper with-navbar with-sidebar with-navbar-fixed-bottom", data-sidebar-hidden="hidden") {
                div(class="sticky-alerts")
                nav(class="navbar") {
                    button(class="btn btn-sm", type="button", on:click=handle_sidebar_click){ "Sidebar" }
                    button(class="btn btn-sm", type="button", on:click=handle_darkmode_click){ "Dark Mode" }
                }
                div(class="sidebar-overlay")
                div(class="sidebar")
                div(class="content-wrapper") {
                    C(0)
                }
                nav(class="navbar navbar-fixed-bottom")
            }
        }
    } else {
        template! {
            div(ref=wrapper_ref, class="page-wrapper with-navbar with-sidebar with-navbar-fixed-bottom") {
                div(class="sticky-alerts")
                nav(class="navbar") {
                    button(class="btn btn-sm", type="button", on:click=handle_sidebar_click){ "Sidebar" }
                    button(class="btn btn-sm", type="button", on:click=handle_darkmode_click){ "Dark Mode" }
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
}
