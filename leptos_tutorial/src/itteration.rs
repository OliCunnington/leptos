use leptos::prelude::*;


#[component]
pub fn ItterateStaticViews() -> impl IntoView {
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
        <ul>{counter_buttons}</ul>
    }
}