use leptos::prelude::*;
use leptos::{ev::SubmitEvent};

#[component]
pub fn ControlledInput() -> impl IntoView {

    let (name, set_name) = signal("Controlled".to_string());

    view! {
        <input type="text"
            on:input:target=move |ev| {
                set_name.set(ev.target().value());
            }
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
pub fn BoundInputs() -> impl IntoView {
    let (name, set_name) = signal("Controlled".to_string());
    let email = RwSignal::new("".to_string());
    let favorite_color = RwSignal::new("red".to_string());
    let spam_me = RwSignal::new(true);

    view! {
        <input type="text"
            bind:value=(name, set_name)
        />
        <input type="email"
            bind:value=email
        />
        <label>
            "Please send me lots of spam email."
            <input type="checkbox"
                bind:checked=spam_me
            />
        </label>
        <fieldset>
            <legend>"Favorite color"</legend>
            <label>
                "Red"
                <input
                    type="radio"
                    name="color"
                    value="red"
                    bind:group=favorite_color
                />
            </label>
            <label>
                "Green"
                <input
                    type="radio"
                    name="color"
                    value="green"
                    bind:group=favorite_color
                />
            </label>
            <label>
                "Blue"
                <input
                    type="radio"
                    name="color"
                    value="blue"
                    bind:group=favorite_color
                />
            </label>
        </fieldset>
        <p>"Your favorite color is " {favorite_color} "."</p>
        <p>"Name is: " {name}</p>
        <p>"Email is: " {email}</p>
        <Show when=move || spam_me.get()>
            <p>"You’ll receive cool bonus content!"</p>
        </Show>
    }
}

#[component]
pub fn UncontrolledInputs() -> impl IntoView {
    let (name, set_name) = signal("Uncontrolled".to_string());

    let input_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {

        ev.prevent_default();

        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();
        set_name.set(value);
    };

    view! {
        <form on:submit=on_submit> 
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

#[component]
pub fn TextAreaWithInput() -> impl IntoView {

    let (some_value, set_some) = signal("text area".to_string());

    view! {
        <textarea
            prop:value=move || some_value.get()
            on:input:target=move |ev| set_some.set(ev.target().value())
        >
            {some_value}
        </textarea>
    }
}

#[component]
pub fn SelectWithInput() -> impl IntoView {
    let (value, set_value) = signal(0i32);
    view! {
        <select
            on:change:target=move |ev| {
               set_value.set(ev.target().value().parse().unwrap());
            }
            prop:value=move || value.get().to_string()
        >
            <option value="0">"0"</option>
            <option value="1">"1"</option>
            <option value="2">"2"</option>
        </select>
        // a button that will cycle through the options
        <button on:click=move |_| set_value.update(|n| {
            if *n == 2 {
            *n = 0;
            } else {
            *n += 1;
            }
        })>
            "Next Option"
        </button>
    }
}
