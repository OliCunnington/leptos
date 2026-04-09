use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_tutorial::*;

fn main() {
    mount_to_body(App);
}


#[component]
fn App() -> impl IntoView {
    

    view! {

        <basic_component::BasicComponent/>

        <dynamic_attributes::DynamicClass/>
        <dynamic_attributes::DynamicStyle/>

        <components_and_props::ProgressBarProp/>

        <passing_children_to_components::TakesChildren render_prop=|| view! { <p>"Hi, there!"</p> }>
            // these get passed to `children`
            "Some text"
            <span>"A span"</span>
            <div style="text-aling:center">
                <div style="display:inline-block;width:auto;background:red"><button>red</button></div>
                <div style="display:inline-block;width:auto;background:blue"><button>blue</button></div>
                <div style="display:inline-block;width:auto;background:green"><button>green</button></div>
            </div>
        </passing_children_to_components::TakesChildren>

        <itteration::ItterateStaticViews/>
        <itteration::ItterateDynamicList initial_length=5/>

    }
}





