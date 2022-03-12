use sycamore::prelude::*;

use crate::todosapp::{app_state::AppState, filter::{TodoFilter, Filter}};

#[component]
pub fn Footer<G: Html>(ctx: ScopeRef) -> View<G> {
    let app_state = ctx.use_context::<AppState>();

    let items_text = || match app_state.todos_left() {
        1 => "item",
        _ => "items",
    };

    let has_completed_todos =
        ctx.create_selector(|| app_state.todos_left() < app_state.todos.get().len());

    let handle_clear_completed = |_| app_state.clear_completed();

    view! { ctx,
        footer(class="footer") {
            span(class="todo-count") {
                strong { (app_state.todos_left()) }
                span { " " (items_text()) " left" }
            }
            ul(class="filters") {
                TodoFilter(Filter::All)
                TodoFilter(Filter::Active)
                TodoFilter(Filter::Completed)
            }

            (if *has_completed_todos.get() {
                view! { ctx,
                    button(class="clear-completed", on:click=handle_clear_completed) {
                        "Clear completed"
                    }
                }
            } else {
                View::empty()
            })
        }
    }
}