use sycamore::prelude::*;

#[component]
pub fn Copyright<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        footer(class="info") {
            p { "Double click to edit a todo" }
            p {
                "Orginal Created for Sycamore examples by "
                a(href="https://github.com/lukechu10", target="_blank") { "lukechu10" }
            }
            p {
                "Part of "
                a(href="http://todomvc.com") { "TodoMVC" }
            }
        }
    }
}