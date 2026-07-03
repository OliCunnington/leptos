use leptos::prelude::*;

#[component]
pub fn SSRModes() -> impl IntoView {
    view!{
        <a href="/async">"Async Rendering"</a>
        <a href="/blocking">"Blocking"</a>
        <a href="/inorder">"In Order"</a>
        <a href="/outoforder">"Out of Order"</a>
        <a href="/partial">"Partially Blocking"</a>
        <a href="/synchronous">"Synchronous Rendering"</a>
    }
}