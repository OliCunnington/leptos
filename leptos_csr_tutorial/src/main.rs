use leptos::mount::mount_to_body;
use leptos::prelude::*;
// use leptos_tutorial::*;
use leptos_router::components::{Router, Route, Routes, ParentRoute};
use leptos_router::path;

mod building_ui;
mod reactivity;
mod async_examples;
mod crud;
mod global_state_management;

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
                <a href="/crud">"CRUD"</a>
            </nav>
            <main>
                <Routes fallback=|| view! { <h1>"Not Found"</h1> }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/building_ui") view=BuildingUI />
                    <Route path=path!("/reactivity") view=Reactivity />
                    <Route path=path!("/testing") view=Testing />
                    <Route path=path!("/async") view=AsyncExamples />
                    <Route path=path!("/projecting_children") view=ProjectingChildren />
                    <Route path=path!("/global_state_management") view=GlobalStateManagement />
                    <Route path=path!("/routing") view=Routing />
                    <ParentRoute path=path!("/crud") view=crud::products::Products>
                        <Route path=path!(":id") view=crud::products::ProductExpanded />
                        <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
                    </ParentRoute>
                    <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>

    }
}

#[component]
pub fn Home() -> impl IntoView {
    view!{}
}

#[component]
pub fn BuildingUI() -> impl IntoView {
    view!{
        <building_ui::basic_component::BasicComponent/>

        <building_ui::dynamic_attributes::DynamicClass/>
        <building_ui::dynamic_attributes::DynamicStyle/>

        <building_ui::components_and_props::ProgressBarProp/>

        <building_ui::itteration::ItterateStaticViews/>
        <building_ui::itteration::ItterateDynamicList initial_length=5/>

        <building_ui::itterating_over_complex_data::ItterateOverComplexData />
        <building_ui::itterating_over_complex_data::NestedSignalItterateOverComplexData />
        <building_ui::itterating_over_complex_data::MemoizedSlicesItterateOverComplexData />
        <building_ui::itterating_over_complex_data::StoresItterateOverComplexData />
        
        <building_ui::forms_and_input::ControlledInput />
        <building_ui::forms_and_input::BoundInputs />
        <building_ui::forms_and_input::UncontrolledInputs />
        <building_ui::forms_and_input::TextAreaWithInput />
        <building_ui::forms_and_input::SelectWithInput />

        <building_ui::control_flow::IfStatement />
        <building_ui::control_flow::OptionView />
        <building_ui::control_flow::MatchStatement />
        <building_ui::control_flow::ShowStatement />

        <building_ui::errors::NumericInputNoErrorHandling />
        <building_ui::errors::NumericInputErrorHandling />

        <building_ui::parent_child_communication::PassingAWriteSignal />
        <building_ui::parent_child_communication::UsingACallback />
        <building_ui::parent_child_communication::UsingEventListener />
        <building_ui::parent_child_communication::ProvidingContext />

        <building_ui::passing_children_to_components::TakesChildren 
            render_prop=|| view! { <p>"Hi, there!"</p> }>
            // these get passed to `children`
            "Some text"
            <span>"A span"</span>
        </building_ui::passing_children_to_components::TakesChildren>
    }
}

#[component]
pub fn Reactivity() -> impl IntoView {
    view!{
        <reactivity::working_with_signals::ClonesAndReplacesVec />
        <reactivity::working_with_signals::ModifyInPlaceVec />
        <reactivity::working_with_signals::BIsAFunctionOfA />
        <reactivity::working_with_signals::CIsAfunctionOfAAndB />
        <reactivity::working_with_signals::AAndBAreIndependentSignals />

        <reactivity::effects::ExampleWithEffect />
        <reactivity::effects::EffectAsAbstraction />
    }
}

#[component]
pub fn Testing() -> impl IntoView {
    view!{
        // needs review - seems to want its own shit
    }
}

#[component]
pub fn AsyncExamples() -> impl IntoView {
    view!{
        <async_examples::loading_data_with_resources::LocalResourcesExample />

        <async_examples::suspense_components::SuspenseComponentExample />

        <async_examples::transition_example::TransitionExample />

        <async_examples::mutating_data_with_actions::AsyncActionExample />
    }
}

#[component]
pub fn ProjectingChildren() -> impl IntoView {
    view!{
        // what the fuck even is that <F, IV> -> impl ... where ...
    }
}

#[component]
pub fn GlobalStateManagement() -> impl IntoView {
    view!{
        <global_state_management::global_state_management::PassingSignalsThroughContext />

        <global_state_management::global_state_management::GlobalStateStoreExample />
    }
}

#[component]
pub fn Routing() -> impl IntoView {
    view!{
        // this needs to be done in main route?
        // nest the examples ?
        // #[component(transparent)]
    }
}
