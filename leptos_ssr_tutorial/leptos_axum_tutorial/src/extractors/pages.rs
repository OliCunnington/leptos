use leptos::prelude::*;
use leptos_router::nested_router::Outlet;

pub fn Extractors() -> impl IntoView {
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