use leptos::prelude::*;
use leptos_router::components::{Router, Route, Routes, ParentRoute};
use leptos_router::MatchNestedRoutes;
use leptos_router::any_nested_route::IntoAnyNestedRoute;

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

//outlet example
// required for rendering of nested child view...
#[component]
pub fn ContactList() -> impl IntoView {
  let contacts = vec![{"id":0,"name":"alice"},{"id":1,"name":"bob"}, {"id":2,"name":"charles"}];

  view! {
    <div style="display: flex">
      // the contact list
      <For each=contacts
        key=|contact| contact.id
        children={move |contact| {view!{<p>{contact}</p>}}}
      />
      // the nested child, if any
      // don’t forget this!
      <Outlet/>
    </div>
  }
}

#[component]
pub fn ContactInfo() -> impl IntoView {
    view!{
        <h1>"ContactInfo"</h1>
    }
}

#[component]
pub fn EmailAndPhone() -> impl IntoView {
    view!{
        <h1>"EmailAndPhone"</h1>
    }
}

#[component]
pub fn Address() -> impl IntoView {
    view!{
        <h1>"Address"</h1>
    }
}

#[component]
pub fn Messages() -> impl IntoView {
    view!{
        <h1>"Messages"</h1>
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
            <Route path=path!("/routing/nested") view=Home/>
            <ParentRoute path=path!("/routing/nested/users") view=Users>
                <Route path=path!(":id") view=UserProfile/>
                <Route path=path!("") view=NoUser/> 
            </ParentRoute>
            <Route path=path!("/routing/nested/*any") view=|| view! { <h1>"Not Found"</h1> }/>
            </Routes>
        </main>
      </Router>
    }
}

#[component]
pub fn DeeperNestedRoutesExample() -> impl IntoView {
    view! {
      <Router>
        <nav>
          /* ... */
        </nav>
        <main>
            <Routes fallback=|| "Not found.">
                <ParentRoute path=path!("/routing/nested/deeper/contacts") view=ContactList>
                    <ParentRoute path=path!(":id") view=ContactInfo>
                        <Route path=path!("") view=EmailAndPhone/>
                        <Route path=path!("address") view=Address/>
                        <Route path=path!("messages") view=Messages/>
                    </ParentRoute>
                    <Route path=path!("") view=|| view! {
                        <p>"Select a contact to view more info."</p>
                    }/>
                </ParentRoute>
            </Routes>
        </main>
      </Router>
    }
}

#[component]
pub fn MultiComponentRoutesExample() -> impl IntoView {
    view! {
      <Router>
        <Routes fallback=|| "Not found.">
          <ParentRoute path=path!("/routing/nested/multi/contacts") view=ContactList>
            <ContactInfoRoutes/>
            <Route path=path!("") view=|| view! {
              <p>"Select a contact to view more info."</p>
            }/>
          </ParentRoute>
        </Routes>
      </Router>
    }
}

#[component(transparent)]
fn ContactInfoRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
      <ParentRoute path=path!(":id") view=ContactInfo>
        <Route path=path!("") view=EmailAndPhone/>
        <Route path=path!("address") view=Address/>
        <Route path=path!("messages") view=Messages/>
      </ParentRoute>
    }
    .into_inner()
    .into_any_nested_route()
}
