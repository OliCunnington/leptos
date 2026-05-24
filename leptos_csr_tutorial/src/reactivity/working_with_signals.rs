use leptos::prelude::*;

#[component]
pub fn ClonesAndReplacesVec() -> impl IntoView {
    let (names, set_names) = signal(Vec::new());
    if names.get().is_empty() { //clones here
        set_names.set(vec!["Alice".to_string()]); //replaces here
    }
    view!{
        {names}
    }
}

#[component]
pub fn ModifyInPlaceVec() -> impl IntoView {
    let (names, set_names) = signal(Vec::new());
    if names.read().is_empty() { //references
        set_names.write().push("Alice".to_string()); //mutates
    }
    view!{
        {names}
    }
}

#[component]
pub fn BIsAFunctionOfA() -> impl IntoView {
    // A
    let (count, set_count) = signal(1);
    // B is a function of A
    let derived_signal_double_count = move || count.get() * 2;
    // B is a function of A
    let memoized_double_count = Memo::new(move |_| count.get() * 2);
    view!{
        <p>{count}</p>
        <p>{derived_signal_double_count}</p>
        <p>{memoized_double_count}</p>
    }
}

#[component]
pub fn CIsAfunctionOfAAndB() -> impl IntoView {
    // A
    let (first_name, set_first_name) = signal("Bridget".to_string());
    // B
    let (last_name, set_last_name) = signal("Jones".to_string());
    // C is a function of A and B
    let full_name = move || format!("{} {}", &*first_name.read(), &*last_name.read());
    view!{
        <p>{first_name}</p>
        <p>{last_name}</p>
        <p>{full_name}</p>
    }
}

#[component]
pub fn AAndBAreIndependentSignals() -> impl IntoView {
    // A
    let (age, set_age) = signal(32);
    // B
    let (favorite_number, set_favorite_number) = signal(42);
    // use this to handle a click on a `Clear` button
    let clear_handler = move |_| {
    // update both A and B
        set_age.set(0);
        set_favorite_number.set(0);
    };
    view!{
        <p>{age} : {favorite_number}</p>
        <button on:click=clear_handler>"Clear"</button>
    }
}