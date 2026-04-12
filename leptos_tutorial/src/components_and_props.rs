use leptos::prelude::*;

#[component]
pub fn ProgressBarProp() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button on:click=move |_| *set_count.write() += 1>
            "Click me"
        </button>
        <br/>
        <ProgressBar progress=count/>
        <ProgressBar progress=Signal::derive(double_count)/>
    }
}

#[component]
fn ProgressBar(
    #[prop(optional)]
    min: u16,
    #[prop(default = 100)]
    max: u16,
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView
{
    view! {
        <progress
            min=min
            max=max
            value=progress
        />
        <br/>
    }
}

//spreading (?!?!) {..}
// you can create attribute lists by using the view macro with a spread {..} as the tag name
// let spread_onto_component = view! {
//     <{..} aria-label="a component with attribute spreading"/>
// };


// view! {
//     // attributes that are spread onto a component will be applied to *all* elements returned as part of
//     // the component's view. to apply attributes to a subset of the component, pass them via a component prop
//     <ComponentThatTakesSpread
//         // plain identifiers are for props
//         some_prop="foo"
//         another_prop=42

//         // the class:, style:, prop:, on: syntaxes work just as they do on elements
//         class:foo=true
//         style:font-weight="bold"
//         prop:cool=42
//         on:click=move |_| alert("clicked ComponentThatTakesSpread")

//         // to pass a plain HTML attribute, prefix it with attr:
//         attr:id="foo"

//         // or, if you want to include multiple attributes, rather than prefixing each with
//         // attr:, you can separate them from component props with the spread {..}
//         {..} // everything after this is treated as an HTML attribute
//         title="ooh, a title!"

//         // we can add the whole list of attributes defined above
//         {..spread_onto_component}
//     />
// }

fn spread_onto_component() -> impl Attribute {
    view!{
        <{..} aria-label="a component with attribute spreading"/>
    }
}

#[component]
pub fn SpreadOntoComponent() -> impl IntoView {
    view!{
        <button {..spread_onto_component()}>Spread</button>
    }
}