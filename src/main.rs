use sycamore::prelude::*;

mod layouts;
mod components;

#[component(App<G>)]
fn app() -> Template<G> {
    template! {
        layouts::base::BaseLayout()
        // components::Hello()
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|| template! { App() });
}
