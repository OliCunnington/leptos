use leptos::prelude::*

#[component]
pub fn SSRModes() -> impl IntoView {
    view!{
        <a href="">"Async Rendering"</a>
        <a href="">"Blocking"</a>
        <a href="">"In Order"</a>
        <a href="">"Out of Order"</a>
        <a href="">"Partially Blocking"</a>
        <a href="">"Synchronous Rendering"</a>
    }
}