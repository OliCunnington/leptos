use leptos::prelude::*;

#[component]
pub fn IfStatement() -> impl IntoView {
    let (value, set_value) = signal(0);
    let is_odd = move || value.get() % 2 != 0;

    view! {
        <button on:click=move |_| {
                *set_value.write() += 1;
            }
        >
            "inc"
        </button>
        <p>
            {move || if is_odd() {
                    "Odd"
                } else {
                    "Even"
                }
            }
        </p>
    }
}

#[component]
pub fn OptionView() -> impl IntoView {
    let message = move || {
        if is_odd() {
            Some("Ding ding ding!")
        } else {
            None
        }
    };
    let (value, set_value) = signal(0);
    let is_odd = move || value.get() % 2 != 0;
    view! {
        <button on:click=move |_| {
                *set_value.write() += 1;
            }
        >
            "inc"
        </button>
        <p>{message}</p>
    }
    //shorter option
    // let message = move || is_odd().then(|| "Ding ding ding!");
    // view! {
    //     <p>{message}</p>
    // }
}

#[component]
pub fn MatchStatement() -> impl IntoView {
    let (value, set_value) = signal(0);
    let is_odd = move || value.get() % 2 != 0;
    let message = move || {
        match value.get() {
            0 => "Zero",
            1 => "One",
            n if is_odd() => "Odd",
            _ => "Even"
        }
    };
    view! {        
        <button on:click=move |_| {
                *set_value.write() += 1;
            }
        >
            "inc"
        </button>
        <p>{message}</p>
    }
}