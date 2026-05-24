use leptos::prelude::*;
use leptos::*;

#[component]
pub fn ClosuresEverywhere() -> impl IntoView {
    // a signal holds a value, and can be updated
    let (count, set_count) = signal(0);

    // a derived signal is a function that accesses other signals
    let double_count = move || count.get() * 2;
    let count_is_odd = move || count.get() & 1 == 1;
    let text = move || if count_is_odd() {
        "odd"
    } else {
        "even"
    };

    // an effect automatically tracks the signals it depends on
    // and reruns when they change
    Effect::new(move |_| {
        logging::log!("text = {}", text());
    });

    view! {
        <p>{move || text().to_uppercase()}</p>
    }
}

#[component]
pub fn SimpleCounter() -> impl IntoView {
    let (value, set_value) = signal(0);

    let increment = move |_| *set_value.write() += 1;

    view! {
        <button on:click=increment>
            {value}
        </button>
    }
}


