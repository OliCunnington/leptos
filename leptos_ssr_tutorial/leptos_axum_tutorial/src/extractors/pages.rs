use leptos::prelude::*;
use leptos_router::nested_router::Outlet;

#[component]
pub fn ExtractorNav() -> impl IntoView {
    view!{
        <main>
            <nav>
                <a href="/extractors/extractors">"Extractors"</a>
                <a href="/extractors/states">"States"</a>
            </nav>
            <Outlet/>
        </main>
    }
}