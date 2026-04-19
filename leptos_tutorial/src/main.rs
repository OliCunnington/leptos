use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_tutorial::*;
use leptos_router::components::{Router, Route, Routes};

mod building_ui;
mod reactivity;

fn main() {
    mount_to_body(App);
}


#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <a href="/">"Home"</a>
                <a href="/building_ui">"Building UI"</a>
                <a href="/reactivity">"Reactivity"</a>
                <a href="/testing">"Testing"</a>
                <a href="/async">"Async"</a>
                <a href="/projecting_children">"Projecting Children"</a>
                <a href="/global_state_management">"Global State Management"</a>
                <a href="/routing">"Routing"</a>
            </nav>
            <main>
                <Routes fallback=|| view! { <h1>"Not Found"</h1> }>
                    <Route path=path!("/building_ui") view= />
                    <Route path=path!("/reactivity") view= />
                    <Route path=path!("/testing") view= />
                    <Route path=path!("/async") view= />
                    <Route path=path!("/projecting_children") view= />
                    <Route path=path!("/global_state_management") view= />
                    <Route path=path!("/routing") view= />
                    <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>

        // <building_ui::basic_component::BasicComponent/>

        // <building_ui::dynamic_attributes::DynamicClass/>
        // <building_ui::dynamic_attributes::DynamicStyle/>

        // <building_ui::components_and_props::ProgressBarProp/>

        // <building_ui::passing_children_to_components::TakesChildren render_prop=|| view! { <p>"Hi, there!"</p> }>
        //     // these get passed to `children`
        //     "Some text"
        //     <span>"A span"</span>
        // </building_ui::passing_children_to_components::TakesChildren>

        // <building_ui::itteration::ItterateStaticViews/>
        // <building_ui::itteration::ItterateDynamicList initial_length=5/>
    }
}





