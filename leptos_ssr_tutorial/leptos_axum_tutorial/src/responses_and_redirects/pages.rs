use leptos::prelude::*;
use leptos_router::nested_router::Outlet;

#[component]
pub fn ResponseOptionsNav() -> impl IntoView {
    view!{
        <main>
            <nav>
                <a href="/responses_and_redirects/response_options">"Response Options"</a>
                <a href="/responses_and_redirects/redirects">"Redirects"</a>
            </nav>
            <Outlet/>
        </main>
    }
}