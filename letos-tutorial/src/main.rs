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
    let (__count, __set_count) = signal(0);

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
    // }

    // view! {
        <button on:click=move |_| *__set_count.write() += 1>
            "Click me"
        </button>
        // now we use our component!
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


/// Displays a `render_prop` and some children within markup.
#[component]
pub fn TakesChildren<F, IV>(
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` can take one of several different types, each of which
    /// is a function that returns some view type
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

// Store -> products view, add to basket, make order
// pagnation... 9 per etc etc
//  <style>
//      .product_card {
//          margin: 5;
//          padding: 3;
//          border-style: 2px ridge black;
//          border-radius: 5;
//      }
//  </style>
//  <div class="product_card">
//      <image>
//      <p>{product_name}</p>
//      <p>{price}</p>
//      <div class="product_hover_over">[icons]</div>
//  </div>

// Manage Products -> add/remove products
// scroll container
//  <button>Add New</button>
//  <ul class="manage_product">
//      <li>{title} {stock} [button for more, delete, edit]</li>
//  </ul>

// ReStock -> order products (resupply), add stock

// Manage Orders -> process orders
// Stats -> see order and sale stats