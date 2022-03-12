use log::info;
use sycamore::prelude::*;
use web_sys::{Event, HtmlElement};

use crate::common::{
    cookie::{create_cookie, read_cookie},
    hotkeys::init_hotkeys,
    utils::to_array,
};

pub fn BaseLayout<G: Html>(Comp: &dyn Fn(ScopeRef, i32) -> View<G>,) -> impl Fn(ScopeRef, ()) -> View<G> + '_ {
    move |ctx, _| {
        let dark_mode_state = ctx.create_signal(true);
        let show_sidebar_state = ctx.create_signal(true);

        let wrapper_ref = ctx.create_node_ref();

        let handle_sidebar_click = move |event: Event| {
            info!("Sidebar toggle");

            let wrapper = wrapper_ref.get::<DomNode>().unchecked_into::<HtmlElement>();
            if wrapper.get_attribute("data-sidebar-hidden").is_some() {
                wrapper
                    .remove_attribute("data-sidebar-hidden")
                    .expect("Failed to remove attribute");
                show_sidebar_state.set(true);
            } else {
                wrapper
                    .set_attribute("data-sidebar-hidden", "hidden")
                    .expect("Failed to set attribute.");
                show_sidebar_state.set(false);
            }

            event.stop_propagation();
        };

        let handle_darkmode_click = move |event: Event| {
            info!("Dark toggle");
            let window = web_sys::window().expect("global window does not exists");
            let document = window.document().expect("expecting a document on window");
            let body = document
                .body()
                .expect("document expect to have have a body");
            let tokens = body.class_list();
            if tokens.contains("dark-mode") {
                body.class_list()
                    .remove(&to_array(&["dark-mode"]))
                    .expect("Failed to remove dark mode");
                dark_mode_state.set(false);
                create_cookie("halfmoon_preferredMode", "light-mode", 365);
            } else {
                tokens
                    .add(&to_array(&["dark-mode"]))
                    .expect("Failed to add dark mode ");
                body.class_list().set_value(&tokens.value());
                dark_mode_state.set(true);
                create_cookie("halfmoon_preferredMode", "dark-mode", 365);
            }

            event.stop_propagation()
        };

        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let client_width = document.document_element().unwrap().client_width();

        // Set dark mode
        // Automatically set preferred theme
        // But only if one of the data-attribute is provided
        let theme = read_cookie("halfmoon_preferredMode");

        match theme.as_str() {
            "dark-mode" => {
                dark_mode_state.set(true);
            }
            "light-mode" => {
                dark_mode_state.set(false);
            }
            _ => {
                let dark_match = window
                    .match_media("(prefers-color-scheme: dark)")
                    .expect("No media match.");
                match dark_match {
                    Some(value) => {
                        if value.matches() {
                            dark_mode_state.set(true);
                        } else {
                            dark_mode_state.set(false);
                        }
                    }
                    None => {
                        dark_mode_state.set(false);
                    }
                }
            }
        }

        let body = document
            .body()
            .expect("Document expect to have have a body");
        let tokens = body.class_list();

        if *dark_mode_state.get() {
            tokens
                .add(&to_array(&["dark-mode"]))
                .expect("Failed to add dark mode ");
        }

        body.class_list().set_value(&tokens.value());
        init_hotkeys();

        // Hiding sidebar on first load on small screens (unless data-attribute provided)
        // Or on larger screens when sidebar type is overlayed-all --
        // TODO: Complete logic
        if client_width <= 768 {
            view! { ctx,
                div(ref=wrapper_ref, class="page-wrapper with-navbar with-sidebar with-navbar-fixed-bottom with-transitions", data-sidebar-hidden="hidden") {
                    div(class="sticky-alerts")
                    nav(class="navbar") {
                        button(class="btn btn-sm", type="button", on:click=handle_sidebar_click){ "Sidebar" }
                        button(class="btn btn-sm", type="button", on:click=handle_darkmode_click){ "Dark Mode" }
                    }
                    div(class="sidebar-overlay")
                    div(class="sidebar")
                    div(class="content-wrapper") {
                        Comp(0)
                    }
                    nav(class="navbar navbar-fixed-bottom")
                }
            }
        } else {
            view! { ctx,
                div(ref=wrapper_ref, class="page-wrapper with-navbar with-sidebar with-navbar-fixed-bottom with-transitions") {
                    div(class="sticky-alerts")
                    nav(class="navbar") {
                        button(class="btn btn-sm", type="button", on:click=handle_sidebar_click){ "Sidebar" }
                        button(class="btn btn-sm", type="button", on:click=handle_darkmode_click){ "Dark Mode" }
                    }
                    div(class="sidebar-overlay")
                    div(class="sidebar")
                    div(class="content-wrapper") {
                        Comp(1)
                    }
                    nav(class="navbar navbar-fixed-bottom")
                }
            }
        }
    }
}
