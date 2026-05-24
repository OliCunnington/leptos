use leptos::prelude::*;
use leptos::html::p;

// to drop #[component] macro
// define a component as a simple function call
// pub fn counter(initial_value: i32, step: u32) -> impl IntoView { }

// elements are created by calling function with same name as html
// p()

// custom is for custom elements
// custom("my-custom-element")

// add chuldren with .child(), takes single, tuple, array
// p().child((em().child("Big, "), strong().child("bold "), "text"))

// attributes can be added with .attr()
// p().attr("id", "foo")
    // .attr("data-count", move || count.get().to_string())
// or attribute method
// p().id("foo")
    // .attr("data-count", move || count.get().to_string())
     
// class:, prop: and style: can be maped directly,
// event listeners with on(), leptos::ev provides event names to avoid typos and provide type inference
// button()
//     .on(ev::click, move |_| set_count.set(0))
//     .child("Clear")

/// A simple counter view.
// A component is really just a function call: it runs once to create the DOM and reactive system
pub fn counter(initial_value: i32, step: i32) -> impl IntoView {
    let (count, set_count) = signal(initial_value);
    div().child((
        button()
            // typed events found in leptos::ev
            // 1) prevent typos in event names
            // 2) allow for correct type inference in callbacks
            .on(ev::click, move |_| set_count.set(0))
            .child("Clear"),
        button()
            .on(ev::click, move |_| *set_count.write() -= step)
            .child("-1"),
        span().child(("Value: ", move || count.get(), "!")),
        button()
            .on(ev::click, move |_| *set_count.write() += step)
            .child("+1"),
    ))
}

pub fn UseOfComponentPropsBuilder() -> impl IntoView {
    let (value, set_value) = signal(0);

    //TODO wrap in div... add button?
    Show(
        ShowProps::builder()
            .when(move || value.get() > 5)
            .fallback(|| p().child("I will appear if `value` is 5 or lower"))
            .children(ToChildren::to_children(|| {
                p().child("I will appear if `value` is above 5")
            }))
            .build(),
    )
}

pub fn DirectBuildWithPropStruct() -> impl IntoView {
    let (value, set_value) = signal(0);

    //TODO wrap in div... add button?
    Show(ShowProps {
        when: move || value.get() > 5,
        fallback: (|| p().child("I will appear if `value` is 5 or lower")).into(),
        children: ToChildren::to_children(|| p().child("I will appear if `value` is above 5")),
    })
}