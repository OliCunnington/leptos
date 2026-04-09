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