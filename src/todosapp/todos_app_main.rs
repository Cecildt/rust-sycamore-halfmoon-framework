use sycamore::prelude::*;

use crate::todosapp::app_state::AppState;
use crate::todosapp::filter::Filter;

use super::copyright;
use super::footer;
use super::header;
use super::list;

// Orginal Todo code from Sycamore example split into multiple files and HTML style changes
#[component]
pub fn TodosApp<G: Html>(ctx: ScopeRef, _prop: i32) -> View<G> {
    // Initialize application state
    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .expect("user has not enabled localStorage");

    let todos = if let Ok(Some(app_state)) = local_storage.get_item(KEY) {
        serde_json::from_str(&app_state).unwrap_or_else(|_| create_rc_signal(Vec::new()))
    } else {
        create_rc_signal(Vec::new())
    };

    let app_state = AppState {
        todos,
        filter: create_rc_signal(Filter::get_filter_from_hash()),
    };
    
    ctx.provide_context(app_state);
    // Set up an effect that runs a function anytime app_state.todos changes
    ctx.create_effect(move || {
        let app_state = ctx.use_context::<AppState>();
        for todo in app_state.todos.get().iter() {
            todo.track();
        }
        local_storage
            .set_item(
                KEY,
                &serde_json::to_string(app_state.todos.get().as_ref()).unwrap(),
            )
            .unwrap();
    });

    view! { ctx,
        div(class="todomvc-wrapper") {
            section(class="todoapp") {
                header::Header()
                list::List()
                footer::Footer()
            }
            copyright::Copyright()
        }
    }
}

const KEY: &str = "todos-sycamore";
