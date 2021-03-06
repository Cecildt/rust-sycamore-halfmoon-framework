use sycamore::prelude::*;

#[allow(dead_code)]
#[component]
pub fn HalfmoonStarter<G: Html>(ctx: ScopeRef, _prop: i32) -> View<G> {
    view! { ctx,
        div(class="w-full h-full d-flex align-items-center justify-content-center") {
            div(class="content"){
                h1(class="content-title") { "Halfmoon starter links" }
                div{
                    a(class="btn btn-link px-0", href="https://www.gethalfmoon.com", target="_blank") { "Halfmoon website" }
                }
                div{
                    a(class="btn btn-link px-0", href="https://www.gethalfmoon.com/docs", target="_blank") { "Halfmoon docs" }
                }
                div{
                    a(class="btn btn-link px-0", href="https://www.gethalfmoon.com/docs/page-building/#starter-template-generator", target="_blank") { "Starter template generator" }
                }
                div{
                    a(class="btn btn-link px-0", href="https://www.twitter.com/halfmoonui", target="_blank") { "Follow on Twitter for updates" }
                }
            }
        }
    }
}