use gloo_timers::future::TimeoutFuture;
use leptos::{html::Input, prelude::*};
use uuid::Uuid;

// Action::new() takes an async function with a single argument
// The input is always a single type. If you want to pass in multiple arguments, you can do it with a struct or tuple.

// // if there's a single argument, just use that
// let action1 = Action::new(|input: &String| {
//    let input = input.clone();
//    async move { todo!() }
// });

// // if there are no arguments, use the unit type `()`
// let action2 = Action::new(|input: &()| async { todo!() });

// // if there are multiple arguments, use a tuple
// let action3 = Action::new(
//   |input: &(usize, String)| async { todo!() }
// );


// Here we define an async function
// This could be anything: a network request, database read, etc.
// Think of it as a mutation: some imperative async action you run,
// whereas a resource would be some async data you load
async fn add_todo(text: &str) -> Uuid {
    _ = text;
    // fake a one-second delay
    // SendWrapper allows us to use this !Send browser API; don't worry about it
    send_wrapper::SendWrapper::new(TimeoutFuture::new(1_000)).await;
    // pretend this is a post ID or something
    Uuid::new_v4()
}

#[component]
pub fn AsyncActionExample() -> impl IntoView {
    // an action takes an async function with single argument
    // it can be a simple type, a struct, or ()
    let add_todo = Action::new(|input: &String| {
        // the input is a reference, but we need the Future to own it
        // this is important: we need to clone and move into the Future
        // so it has a 'static lifetime
        let input = input.to_owned();
        async move { add_todo(&input).await }
    });

    // actions provide a bunch of synchronous, reactive variables
    // that tell us different things about the state of the action
    let submitted = add_todo.input();
    let pending = add_todo.pending();
    let todo_id = add_todo.value();

    let input_ref = NodeRef::<Input>::new();

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default(); // don't reload the page...
                let input = input_ref.get().expect("input to exist");
                add_todo.dispatch(input.value());
            }
        >
            <label>
                "What do you need to do?"
                <input type="text"
                    node_ref=input_ref
                />
            </label>
            <button type="submit">"Add Todo"</button>
        </form>
        <p>{move || pending.get().then_some("Loading...")}</p>
        <p>
            "Submitted: "
            <code>{move || format!("{:#?}", submitted.get())}</code>
        </p>
        <p>
            "Pending: "
            <code>{move || format!("{:#?}", pending.get())}</code>
        </p>
        <p>
            "Todo ID: "
            <code>{move || format!("{:#?}", todo_id.get())}</code>
        </p>
    }
}
