use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}


#[component]
fn App() -> impl IntoView {
    
    let (count, set_count) = signal(0);
    let (_count, _set_count) = signal(0);
    let double_count = move || count.get() * 2;
    let (__count, __set_count) = signal(0);

    view! {
        <button
            on:click= //move |_| set_count.set(3)
                move |_| {
                    *set_count.write() += 1;
                }
            class:red=move || count.get() % 2 == 1
        >
            "Click me: "
            {count}
        </button>
        <progress
            max="50"
            value=double_count
        />
        <p>
            "Double Count: "
            {double_count}
        </p>
        <button
            on:click=move |_| {
                *_set_count.write() += 10;
            }
            style="position: absolute"
            style:left=move || format!("{}px", _count.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", _count.get(), 100)
            style:max-width="400px"
            style=("--columns", move || _count.get().to_string())
        >
            "Click to Move"
        </button>
    // }

    // view! {
        <button on:click=move |_| *__set_count.write() += 1>
            "Click me"
        </button>
        <br/>
        <ProgressBar progress=__count/>
        <br/>
        <ProgressBar max=25 progress=__count/>
    // }

    // view! {
        <TakesChildren render_prop=|| view! { <p>"Hi, there!"</p> }>
            // these get passed to `children`
            "Some text"
            <span>"A span"</span>
            <div style="text-aling:center">
                <div style="display:inline-block;width:auto;background:red"><button>red</button></div>
                <div style="display:inline-block;width:auto;background:blue"><button>blue</button></div>
                <div style="display:inline-block;width:auto;background:green"><button>green</button></div>
            </div>
        </TakesChildren>
    }
}

#[component]
fn ProgressBar(
    //#[prop(optional)]
    #[prop(default = 100)]
    max: u16,
    progress: ReadSignal<i32>
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}


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


#[component]
fn PageSwitch() -> impl IntoView {
    view! {
        <div class="nav_bar">
            <button>Store/Customer</button>
            <button>Manage/Vendor</button>
            <button>Sales Stats</button>
        </div>
    }
}