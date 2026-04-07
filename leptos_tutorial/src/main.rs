use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_tutorial::*;

fn main() {
    mount_to_body(App);
}


#[component]
fn App() -> impl IntoView {
    

    // create a list of 5 signals
    let length = 5;
    let counters = (1..=length).map(|idx| RwSignal::new(idx));

    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button
                        on:click=move |_| *count.write() += 1
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();

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

        <ul>{counter_buttons}</ul>

    }
}





