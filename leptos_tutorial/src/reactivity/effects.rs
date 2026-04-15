use leptos::prelude::*;
use leptos::*;

#[component]
pub fn ExampleWithEffect() -> impl IntoView {
    let (a, set_a) = signal(0);
    let (b, set_b) = signal(0);

    Effect::new(move |_| {
        // immediately prints "Value: 0" and subscribes to `a`
        logging::log!("Value: {}", a.get());    
    });

    view!{
        <p>{a}</p>
        <p>{b}</p>
        <button on:click=move |_| {*set_a.write() += 1}>A</button>
        <button on:click=move |_| {*set_b.write() += 1}>B</button>
    }
}

#[component]
pub fn EffectAsAbstraction() -> impl IntoView {
    let (first, set_first) = signal(String::new());
    let (last, set_last) = signal(String::new());
    let (use_last, set_use_last) = signal(true);

    // this will add the name to the log
    // any time one of the source signals changes
    Effect::new(move |_| {
        logging::log!(
            "{}", if use_last.get() {
                format!("{} {}", first.get(), last.get())
            } else {
                first.get()
            },
        )
    });

    view!{
        <input type="text" placeholder="first"
            on:input:target=move |ev| {
                set_first.set(ev.target().value());
            }
            prop:value=first
        />
        <input type="text" placeholder="last"
            on:input:target=move |ev| {
                set_last.set(ev.target().value());
            }
            prop:value=last
        />
        <label>
            "Use last"
            <input type="checkbox"
                bind:checked=(use_last, set_use_last)
            />
        </label>
    }
}

// let (count, set_count) = signal(0);

// view! {
//     <p>{count}</p>
// }

// essentially wrapped and implemented as follows:

// let (count, set_count) = signal(0);

// // create a DOM element
// let document = leptos::document();
// let p = document.create_element("p").unwrap();

// // create an effect to reactively update the text
// Effect::new(move |prev_value| {
//     // first, access the signal’s value and convert it to a string
//     let text = count.get().to_string();

//     // if this is different from the previous value, update the node
//     if prev_value != Some(text) {
//         p.set_text_content(&text);
//     }

//     // return this value so we can memoize the next update
//     text
// });


// explicit tracking with Effect::watch()
 
// let (num, set_num) = signal(0);

// let effect = Effect::watch(
//     move || num.get(),
//     move |num, prev_num, _| {
//         leptos::logging::log!("Number: {}; Prev: {:?}", num, prev_num);
//     },
//     false,
// );

// set_num.set(1); // > "Number: 1; Prev: Some(0)"

// effect.stop(); // stop watching

// set_num.set(2); // (nothing happens)