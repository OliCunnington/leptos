use leptos::prelude::*;
use leptos_router::nested_router::Outlet;

#[component]
pub fn ServerFunctions() -> impl IntoView {
    view!{
        <main>
            <nav>
                <a href="/server_functions/custom_errors">"Custom Errors"</a>
                <a href="/server_functions/todos">"Server Function Example - Todos"</a>
            </nav>
            <Outlet/>
        </main>
    }
}