use sycamore::prelude::*;

use crate::todosapp::{app_state::AppState, filter::Filter, item::Item};


#[component]
pub fn List<G: Html>(ctx: ScopeRef) -> View<G> {
    let app_state = ctx.use_context::<AppState>();
    let todos_left = ctx.create_selector(|| app_state.todos_left());

    let filtered_todos = ctx.create_memo(|| {
        app_state
            .todos
            .get()
            .iter()
            .filter(|todo| match *app_state.filter.get() {
                Filter::All => true,
                Filter::Active => !todo.get().completed,
                Filter::Completed => todo.get().completed,
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    // We need a separate signal for checked because clicking the checkbox will detach the binding
    // between the attribute and the view.
    let checked = ctx.create_signal(false);
    ctx.create_effect(|| {
        // Calling checked.set will also update the `checked` property on the input element.
        checked.set(*todos_left.get() == 0)
    });

    view! { ctx,
        section(class="main") {
            input(
                id="toggle-all",
                class="toggle-all",
                type="checkbox",
                readonly=true,
                bind:checked=checked,
                on:input=|_| app_state.toggle_complete_all()
            )
            label(for="toggle-all")

            ul(class="todo-list") {
                Keyed {
                    iterable: filtered_todos,
                    view: |ctx, todo| view! { ctx,
                        Item(todo)
                    },
                    key: |todo| todo.get().id,
                }
            }
        }
    }
}