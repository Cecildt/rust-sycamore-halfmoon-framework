use sycamore::prelude::*;

#[component(BaseLayout<G>)]
pub fn base_layout() -> Template<G> {
    template! {
        div(class="page-wrapper with-navbar with-sidebar with-navbar-fixed-bottom") {
            div(class="sticky-alerts")
            nav(class="navbar")
            div(class="sidebar-overlay")
            div(class="sidebar")
            div(class="content-wrapper") {

            }
            nav(class="navbar navbar-fixed-bottom")
        }
    }
}