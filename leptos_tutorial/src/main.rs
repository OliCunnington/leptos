use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_tutorial::*;

mod building_ui;
mod reactivity;

fn main() {
    mount_to_body(App);
}


#[component]
fn App() -> impl IntoView {
    

    view! {

        <building_ui::basic_component::BasicComponent/>

        <building_ui::dynamic_attributes::DynamicClass/>
        <building_ui::dynamic_attributes::DynamicStyle/>

        <building_ui::components_and_props::ProgressBarProp/>

        <building_ui::passing_children_to_components::TakesChildren render_prop=|| view! { <p>"Hi, there!"</p> }>
            // these get passed to `children`
            "Some text"
            <span>"A span"</span>
            <div style="text-aling:center">
                <div style="display:inline-block;width:auto;background:red"><button>red</button></div>
                <div style="display:inline-block;width:auto;background:blue"><button>blue</button></div>
                <div style="display:inline-block;width:auto;background:green"><button>green</button></div>
            </div>
        </building_ui::passing_children_to_components::TakesChildren>

        <building_ui::itteration::ItterateStaticViews/>
        <building_ui::itteration::ItterateDynamicList initial_length=5/>

    }
}





