use sycamore::{prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};

use super::app_state::AppState;

#[component]
pub fn Header<G: Html>(ctx: ScopeRef) -> View<G> {
    let app_state = ctx.use_context::<AppState>();
    let value = ctx.create_signal(String::new());

    let handle_submit = move |event: Event| {
        let event: KeyboardEvent = event.unchecked_into();

        if event.key() == "Enter" {
            let mut task = value.get().as_ref().clone();
            task = task.trim().to_string();

            if !task.is_empty() {
                app_state.add_todo(task);
                value.set("".to_string());
            }
        }
    };

    view! { ctx,
        header(class="header") {
            h1 { "todos" }
            input(class="new-todo",
                placeholder="What needs to be done?",
                bind:value=value,
                on:keyup=handle_submit,
            )
        }
    }
}
