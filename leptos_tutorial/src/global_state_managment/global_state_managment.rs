// The three best approaches to global state are

// Using the router to drive global state via the URL
// Passing signals through context
// Creating a global state struct using stores

use leptos::prelude::*;

#[component]
fn PassingSignalsThroughContext() -> impl IntoView {
    // here we create a signal in the root that can be consumed
    // anywhere in the app.
    let (count, set_count) = signal(0);
    // we'll pass the setter to specific components,
    // but provide the count itself to the whole app via context
    provide_context(count);

    view! {
        // SetterButton is allowed to modify the count
        <SetterButton set_count/>
        // These consumers can only read from it
        // But we could give them write access by passing `set_count` if we wanted
        <FancyMath/>
        // <ListItems/>
    }
}

#[component]
fn SetterButton(set_count: WriteSignal<i32>) -> impl IntoView {
    view {
        <button on:click=move || set_count.write() += 1>"+"</button>
    }
}

/// A component that does some "fancy" math with the global count
#[component]
fn FancyMath() -> impl IntoView {
    // here we consume the global count signal with `use_context`
    let count = use_context::<ReadSignal<u32>>()
        // we know we just provided this in the parent component
        .expect("there to be a `count` signal provided");
    let is_even = move || count.get() & 1 == 0;

    view! {
        <div class="consumer blue">
            "The number "
            <strong>{count}</strong>
            {move || if is_even() {
                " is"
            } else {
                " is not"
            }}
            " even."
        </div>
    }
}

// create and use a Global State Store

use reactive_stores::Store;

#[derive(Clone, Debug, Default, Store)]
struct GlobalState {
    count: i32,
    name: String,
}

#[component]
fn GlobalStateStoreExample() -> impl IntoView {
    provide_context(Store::new(GlobalState::default()));

    // etc.
    view!{
        <GlobalStateCounter/>
    }
}

/// A component that updates the count in the global state.
#[component]
fn GlobalStateCounter() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();

    // this gives us reactive access to the `count` field only
    let count = state.count();

    view! {
        <div class="consumer blue">
            <button
                on:click=move |_| {
                    *count.write() += 1;
                }
            >
                "Increment Global Count"
            </button>
            <br/>
            <span>"Count is: " {move || count.get()}</span>
        </div>
    }
}