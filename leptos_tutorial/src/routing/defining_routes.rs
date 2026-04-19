use leptos::prelude::*;
use leptos_router::components::{Router, Route, Routes};

#[component]
pub fn RouterExample() -> impl IntoView {
    view! {
      <Router>
        <nav>
          /* ... */
        </nav>
        <main>
          // all our routes will appear inside <main>
          <Routes fallback=|| "Not found.">
            <Route path=path!("/routing") view=Home/>
            <Route path=path!("/routing/users") view=Users/>
            <Route path=path!("/routing/users/:id") view=UserProfile/>
            <Route path=path!("/routing/*any") view=|| view! { <h1>"Not Found"</h1> }/>
          </Routes>
        </main>
      </Router>
    }
}

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