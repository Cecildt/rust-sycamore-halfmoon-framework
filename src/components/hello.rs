use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

#[component]
pub fn Hello<G: Html>(ctx: ScopeRef) -> View<G> {
    let name = ctx.create_signal("");

    let displayed_name = || {
        if name.get().is_empty() {
            "World".to_string()
        } else {
            name.get().as_ref().clone().to_string()
        }
    };

    let handle_change = move |event: Event| {
        // let value = event
        // .target()
        // .unwrap()
        // .dyn_into::<HtmlInputElement>()
        // .unwrap()
        // .value().as_str();

        // name.set(value);
    };

    view! {ctx, 
        div {
            h1 {
                "Hello "
                (displayed_name())
                "!"
            }

            input(placeholder="What is your name?", on:input=handle_change)
        }
    }
}