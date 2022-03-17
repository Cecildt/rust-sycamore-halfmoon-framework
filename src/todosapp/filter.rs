use sycamore::{prelude::*};

use super::app_state::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Self::All
    }
}

impl Filter {
    pub fn url(self) -> &'static str {
        match self {
            Filter::All => "#",
            Filter::Active => "#/active",
            Filter::Completed => "#/completed",
        }
    }

    pub fn get_filter_from_hash() -> Self {
        let hash = web_sys::window().unwrap().location().hash().unwrap();

        match hash.as_str() {
            "#/active" => Filter::Active,
            "#/completed" => Filter::Completed,
            _ => Filter::All,
        }
    }
}

#[component]
pub fn TodoFilter<G: Html>(ctx: ScopeRef, filter: Filter) -> View<G> {
    let app_state = ctx.use_context::<AppState>();
    let selected = move || filter == *app_state.filter.get();
    let set_filter = |filter| app_state.filter.set(filter);

    view! { ctx,
        li {
            a(
                class=if selected() { "hyperlink selected" } else { "hyperlink" },
                href=filter.url(),
                on:click=move |_| set_filter(filter),
            ) {
                (format!("{filter:?}"))
            }
        }
    }
}
