use sycamore::prelude::*;

use crate::components::halfmoon_starter;

mod layouts;
mod components;

type BasicLayoutStarter<G> = layouts::base::BaseLayout<G, halfmoon_starter::HalfmoonStarter<G>>;

#[component(App<G>)]
fn app() -> Template<G> {

    template! {
        BasicLayoutStarter()
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|| template! { App() });
}
