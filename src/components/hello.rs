use sycamore::prelude::*;
use web_sys::{Event, HtmlInputElement};
use wasm_bindgen::JsCast;

#[allow(dead_code)]
#[component]
pub fn Hello<G: Html>(ctx: ScopeRef) -> View<G> {
    let name = ctx.create_signal("".to_owned());

    let displayed_name = || {
        if name.get().is_empty() {
            "World".to_string()
        } else {
            name.get().as_ref().clone().to_string()
        }
    };

    let handle_change = move |event: Event| {
        let target = event.target().unwrap();
        let input = target.dyn_into::<HtmlInputElement>().unwrap();
        let value = input.value();
        name.set(value);
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