use leptos::prelude::*;
use gloo_timers::future::TimeoutFuture;

// basic loading screen (fallback)
// let (count, set_count) = signal(0);
// let once = Resource::new(move || count.get(), |count| async move { load_a(count).await });

// view! {
//     <h1>"My Data"</h1>
//     {move || match once.get() {
//         None => view! { <p>"Loading..."</p> }.into_any(),
//         Some(data) => view! { <ShowData data/> }.into_any()
//     }}
// }

// waiting on 2 resources
// let (count, set_count) = signal(0);
// let (count2, set_count2) = signal(0);
// let a = Resource::new(move || count.get(), |count| async move { load_a(count).await });
// let b = Resource::new(move || count2.get(), |count| async move { load_b(count).await });

// view! {
//     <h1>"My Data"</h1>
//     {move || match (a.get(), b.get()) {
//         (Some(a), Some(b)) => view! {
//             <ShowA a/>
//             <ShowA b/>
//         }.into_any(),
//         _ => view! { <p>"Loading..."</p> }.into_any()
//     }}
// }

#[component]
pub fn SuspenseComponentExample() -> impl IntoView {
    let (count, set_count) = signal("A");
    let (count2, set_count2) = signal("B");
    let a = LocalResource::new(
        move || load_a(count.get().to_string())
    );
    let b = LocalResource::new(
        move || load_b(count2.get().to_string())
    );

    view! {
        <h1>"My Data"</h1>
        <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
            <h2>"My Data"</h2>
            <h3>"A"</h3>
            {move || {
                a.get()
                    .map(|a| view! { <ShowA a/> })
            }}
            <h3>"B"</h3>
            {move || {
                b.get()
                    .map(|b| view! { <ShowB b/> })
            }}
        </Suspense>
    }
}

#[component]
pub fn ShowA(a: String) -> impl IntoView{
    view!{
        <p>"A : "{a}</p>
    }
}

#[component]
pub fn ShowB(b: String) -> impl IntoView{
    view!{
        <p>"B : "{b}</p>
    }
}

async fn load_a(name: String) -> String {
    TimeoutFuture::new(1_000).await;
    name
}

async fn load_b(name: String) -> String {
    TimeoutFuture::new(1_000).await;
    name
}


// suspend inside suspense
// allows avoiding null checks in each resource

// view! {
//     <h1>"My Data"</h1>
//     <Suspense
//         fallback=move || view! { <p>"Loading..."</p> }
//     >
//         <h2>"My Data"</h2>
//         {move || Suspend::new(async move {
//             let a = a.await;
//             let b = b.await;
//             view! {
//                 <h3>"A"</h3>
//                 <ShowA a/>
//                 <h3>"B"</h3>
//                 <ShowB b/>
//             }
//         })}
//     </Suspense>
// }

// await element
// combines OnceResource with a Suspense, with no fallback

// async fn fetch_monkeys(monkey: i32) -> i32 {
//     // maybe this didn't need to be async
//     monkey * 2
// }
// view! {
//     <Await
//         // `future` provides the `Future` to be resolved
//         future=fetch_monkeys(3)
//         // the data is bound to whatever variable name you provide
//         let:data
//     >
//         // you receive the data by reference and can use it in your view here
//         <p>{*data} " little monkeys, jumping on the bed."</p>
//     </Await>
// }