use sycamore::prelude::*;

use super::copyright;
use super::footer;
use super::header;
use super::list;

#[component]
pub fn TodosApp<G: Html>(ctx: ScopeRef, _prop: i32) -> View<G> {
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
