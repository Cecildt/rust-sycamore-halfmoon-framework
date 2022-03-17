use sycamore::prelude::*;

use crate::{components::halfmoon_starter::HalfmoonStarter, todosapp::todos_app_main::TodosApp};

mod common;
mod components;
mod layouts;
mod todosapp;

#[allow(dead_code)]
#[component]
fn App<G: Html>(ctx: ScopeRef) -> View<G> {
    let BasicLayoutStarter = layouts::base::BaseLayout(&HalfmoonStarter);
    view! { ctx,
        BasicLayoutStarter()
    }
}

#[allow(dead_code)]
#[component]
fn AppTodos<G: Html>(ctx: ScopeRef) -> View<G> {
    let BasicLayoutTodos = layouts::base::BaseLayout(&TodosApp);
    view! { ctx,
        BasicLayoutTodos()
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|ctx| view! { ctx, AppTodos() });

    //sycamore::render(|ctx| view! { ctx, App() });
}
