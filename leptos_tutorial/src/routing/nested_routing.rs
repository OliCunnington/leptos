use leptos::prelude::*;
use leptos_router::components::{Router, Route, Routes};

#[component]
pub fn Home() -> impl IntoView {
    view!{
        <h1>"Home"</h1>
    }
}

#[component]
pub fn Users() -> impl IntoView {
    view!{
        <h1>"Users"</h1>
    }
}

#[component]
pub fn UserProfile() -> impl IntoView {
    view!{
        <h1>"UserProfile"</h1>
    }
}

#[component]
pub fn NoUser() -> impl IntoView {
    view!{
        <h1>"NoUser"</h1>
    }
}

#[component]
pub fn NestedRoutesExample() -> impl IntoView {
    view! {
      <Router>
        <nav>
          /* ... */
        </nav>
        <main>
            <Routes fallback=|| "Not found.">
            <Route path=path!("/") view=Home/>
            <ParentRoute path=path!("/users") view=Users>
                <Route path=path!(":id") view=UserProfile/>
                <Route path=path!("") view=NoUser/> 
            </ParentRoute>
            <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
            </Routes>
        </main>
      </Router>
    }
}

