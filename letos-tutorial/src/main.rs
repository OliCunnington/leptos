// use leptos::prelude::*;

// fn main() {
//     console_error_panic_hook::set_once();
//     leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> })
// }

use leptos::mount::mount_to_body;

fn main() {
    mount_to_body(App);
}

use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    
    let (count, set_count) = signal(0);
    let (_count, _set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button
            on:click= //move |_| set_count.set(3)
                move |_| {
                    *set_count.write() += 1;
                }
            class:red=move || count.get() % 2 == 1
            // Some CSS class names can’t be directly parsed by the view macro
            // use a tuple syntax: class=("name", value)
            // tuple syntax also allows specifying multiple classes under a single condition
            // class=(["button-20", "rounded"], move || count.get() % 2 == 1)
        >
            "Click me: "
            {count}
        </button>
        <progress
            max="50"
            // we use it once here
            value=double_count
        />
        <p>
            "Double Count: "
            // and again here
            {double_count}
        </p>
        <button
            on:click=move |_| {
                *_set_count.write() += 10;
            }
            // set the `style` attribute
            style="position: absolute"
            // and toggle individual CSS properties with `style:`
            style:left=move || format!("{}px", _count.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", _count.get(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", move || _count.get().to_string())
        >
            "Click to Move"
        </button>
    }
}