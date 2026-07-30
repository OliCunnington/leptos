use leptos::prelude::*;

#[server]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    const INVALID_CREDENTIALS: fn() -> ServerFnError = || -> ServerFnError {
        ServerFnError::ServerError("Invalid credentials".into())
    };

    // pull the DB pool and auth provider from context
    let pool = pool()?;
    let auth = auth()?;

    // check whether the user exists
    let user: User = User::get_from_username(username, &pool)
        .await
        .ok_or_else(INVALID_CREDENTIALS)?;

    // check whether the user has provided the correct password
    match verify(password, &user.password)? {
        // if the password is correct...
        true => {
            // log the user in
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());

            // and redirect to the home page
            leptos_axum::redirect("/");
            Ok(())
        }
        // if not, return an error
        false => Err(INVALID_CREDENTIALS()),
    }
}