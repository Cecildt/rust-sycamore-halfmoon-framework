use sycamore::{prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, KeyboardEvent};

use crate::todosapp::app_state::AppState;

use super::todo::Todo;

#[component]
pub fn Item<G: Html>(ctx: ScopeRef, todo: RcSignal<Todo>) -> View<G> {
    let app_state = ctx.use_context::<AppState>();
    // Make `todo` live as long as the scope.
    let todo = ctx.create_ref(todo);

    let title = || todo.get().title.clone();
    let completed = ctx.create_selector(|| todo.get().completed);
    let id = todo.get().id;

    let editing = ctx.create_signal(false);
    let input_ref = ctx.create_node_ref();
    let value = ctx.create_signal("".to_string());

    let handle_input = |event: Event| {
        let target: HtmlInputElement = event.target().unwrap().unchecked_into();
        value.set(target.value());
    };

    let toggle_completed = |_| {
        todo.set(Todo {
            completed: !todo.get().completed,
            ..todo.get().as_ref().clone()
        });
    };

    let handle_dblclick = move |_| {
        editing.set(true);
        input_ref
            .get::<DomNode>()
            .unchecked_into::<HtmlInputElement>()
            .focus()
            .unwrap();
        value.set(title());
    };

    let handle_blur = move || {
        editing.set(false);

        let mut value = value.get().as_ref().clone();
        value = value.trim().to_string();

        if value.is_empty() {
            app_state.remove_todo(id);
        } else {
            todo.set(Todo {
                title: value,
                ..todo.get().as_ref().clone()
            })
        }
    };

    let handle_submit = move |event: Event| {
        let event: KeyboardEvent = event.unchecked_into();
        match event.key().as_str() {
            "Enter" => handle_blur(),
            "Escape" => {
                input_ref
                    .get::<DomNode>()
                    .unchecked_into::<HtmlInputElement>()
                    .set_value(&title());
                editing.set(false);
            }
            _ => {}
        }
    };

    let handle_destroy = move |_| {
        app_state.remove_todo(id);
    };

    // We need a separate signal for checked because clicking the checkbox will detach the binding
    // between the attribute and the view.
    let checked = ctx.create_signal(false);
    ctx.create_effect(|| {
        // Calling checked.set will also update the `checked` property on the input element.
        checked.set(*completed.get())
    });

    let class = || {
        format!(
            "{} {}",
            if *completed.get() { "completed" } else { "" },
            if *editing.get() { "editing" } else { "" }
        )
    };

    view! { ctx,
        li(class=class()) {
            div(class="view") {
                div(class="custom-checkbox"){
                    input(
                        type="checkbox",
                        on:input=toggle_completed,
                        bind:checked=checked
                    )
                    label(on:dblclick=handle_dblclick) {
                        (title())
                    }
                }                
                div{
                    button(class="btn btn-sm rounded-circle destroy", on:click=handle_destroy){
                        "X"
                    }
                }
            }

            (if *editing.get() {
                view! { ctx,
                    input(ref=input_ref,
                        class="form-control",
                        value=todo.get().title.clone(),
                        on:blur=move |_| handle_blur(),
                        on:keyup=handle_submit,
                        on:input=handle_input,
                    )
                }
            } else {
                View::empty()
            })
        }
    }
}