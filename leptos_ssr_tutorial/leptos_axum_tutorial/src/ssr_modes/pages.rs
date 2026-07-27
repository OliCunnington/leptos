use leptos::prelude::*;
use leptos_router::nested_router::Outlet;

#[component]
pub fn SSRModes() -> impl IntoView {
    view!{
        <main>
            <nav>
                <a href="/ssr_modes/async">"Async Rendering"</a>
                <a href="/ssr_modes/blocking">"Blocking"</a>
                <a href="/ssr_modes/inorder">"In Order"</a>
                <a href="/ssr_modes/outoforder">"Out of Order"</a>
                <a href="/ssr_modes/partial">"Partially Blocking"</a>
                <a href="/ssr_modes/synchronous">"Synchronous Rendering"</a>
            </nav>
            <Outlet/>
        </main>
    }
}