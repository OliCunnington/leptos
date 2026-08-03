use leptos::prelude::*;


#[server]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    todo!()
}

#[component]
fn AddTodo() -> impl IntoView {
    let add_todo = ServerAction::<AddTodo>::new();
    // holds the latest *returned* value from the server
    let value = add_todo.value();
    // check if the server has returned an error
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <ActionForm action=add_todo>
            <label>
                "Add a Todo"
                // `title` matches the `title` argument to `add_todo`
                <input type="text" name="title"/>
            </label>
            <input type="submit" value="Add"/>
        </ActionForm>
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Settings {
    display_name: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct HeftyData {
    first_name: String,
    last_name: String,
    settings: Settings,
}

#[component]
fn ComplexInput() -> impl IntoView {
    let submit = ServerAction::<VeryImportantFn>::new();

    view! {
      <ActionForm action=submit>
        <input type="text" name="hefty_arg[first_name]" value="leptos"/>
        <input
          type="text"
          name="hefty_arg[last_name]"
          value="closures-everywhere"
        />
        <input
          type="text"
          name="hefty_arg[settings][display_name]"
          value="my alias"
        />
        <input type="submit"/>
      </ActionForm>
    }
}

#[server]
async fn very_important_fn(hefty_arg: HeftyData) -> Result<(), ServerFnError> {
    assert_eq!(hefty_arg.first_name.as_str(), "leptos");
    assert_eq!(hefty_arg.last_name.as_str(), "closures-everywhere");
    aseert_eq!(hefty_arg.settings.display_name.as_str(), "my alias");
    Ok(())
}