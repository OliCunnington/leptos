use leptos::prelude::*;

#[component]
pub fn TakesChildren<F, IV>(
    render_prop: F,
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h1><code>"<TakesChildren/>"</code></h1>
        <h2>"Render Prop"</h2>
        {render_prop()}
        <hr/>
        <h2>"Children"</h2>
        {children()}
    }
}

#[slot]
struct Then {
    children: ChildrenFn,
}

#[component]
fn If(
    condition: Signal<bool>,
    // Component slot, should be passed through the <Then slot> syntax
    then_slot: Then,
) -> impl IntoView {
    move || {
        if condition.get() {
            (then_slot.children)().into_any()
        } else {
            ().into_any()
        }
    }
}

#[component]
pub fn SlotExample() -> impl IntoView {
    let (value, set_value) = signal(true);
    view! {
        <If condition=value.into()>
            // The `If` component always expects a `Then` child for `then_slot`
            <Then slot:then_slot>"Show content when a is true"</Then>
        </If>
    }
}

#[component]
pub fn WrapsChildren(children: ChildrenFragment) -> impl IntoView {
    // children() returns a `Fragment`, which has a
    // `nodes` field that contains a Vec<View>
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect::<Vec<_>>();

    view! {
        <h1><code>"<WrapsChildren/>"</code></h1>
        <ul>{children}</ul>
    }
}

#[component]
pub fn WrappedChildren() -> impl IntoView {
    view! {
        <WrapsChildren>
            "A"
            "B"
            "C"
        </WrapsChildren>
    }
}