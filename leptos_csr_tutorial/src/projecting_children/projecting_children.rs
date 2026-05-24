use leptos::prelude::*;

// The problem/consider following (won't compile)
// pub fn NestedShow<F, IV>(fallback: F, children: ChildrenFn) -> impl IntoView
// where
//     F: Fn() -> IV + Send + Sync + 'static,
//     IV: IntoView + 'static,
// {
//     view! {
//         <Show
//             when=|| todo!()
//             fallback=|| ()
//         >
//             <Show
//                 when=|| todo!()
//                 fallback=fallback
//             >
//                 {children()}
//             </Show>
//         </Show>
//     }
// }

// expanded view macro, to understand issue
// Show(
//     ShowProps::builder()
//         .when(|| todo!())
//         .fallback(|| ())
//         .children({
//             // children and fallback are moved into a closure here
//             ::leptos::children::ToChildren::to_children(move || {
//                 Show(
//                     ShowProps::builder()
//                         .when(|| todo!())
//                         // fallback is consumed here
//                         .fallback(fallback)
//                         .children({
//                             // children is captured here
//                             ::leptos::children::ToChildren::to_children(
//                                 move || children(),
//                             )
//                         })
//                         .build(),
//                 )
//             })
//         })
//         .build(),
// )

// solution
#[component]
pub fn NestedShow<F, IV>(fallback: F, children: ChildrenFn) -> impl IntoView
where
    F: Fn() -> IV + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    let fallback = StoredValue::new(fallback);
    let children = StoredValue::new(children);

    view! {
        <Show
            when=|| todo!()
            fallback=|| ()
        >
            <Show
                // check whether user is verified
                // by reading from the resource
                when=move || todo!()
                fallback=move || fallback.read_value()()
            >
                {children.read_value()()}
            </Show>
        </Show>
    }
}

// consider
#[component]
pub fn CloneExample() -> impl IntoView {
    let name = "Alice".to_string();
    view! {
        <Outer>
            <Inner clone:name> // clone here to resolve issue
                <Inmost name=name.clone()/>
            </Inner>
        </Outer>
    }
}

#[component]
pub fn Outer(children: ChildrenFn) -> impl IntoView {
    children()
}

#[component]
pub fn Inner(children: ChildrenFn) -> impl IntoView {
    children()
}

#[component]
pub fn Inmost(name: String) -> impl IntoView {
    view! {
        <p>{name}</p>
    }
}