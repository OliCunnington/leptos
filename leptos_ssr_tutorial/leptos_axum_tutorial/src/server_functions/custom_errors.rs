use leptos::prelude::*;
use server_fn::codec::JsonEncoding;
use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AppError {
    ServerFnError(ServerFnErrorErr),
    DbError(String),
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    name: String,
    email: String
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ServerFnError(e)  => write!(f, "{}", e.to_string()),
            AppError::DbError(e)        => write!(f, "{}", e.to_string())
        }
    }
}

impl FromServerFnError for AppError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        AppError::ServerFnError(value)
    }
}

#[server]
pub async fn create_user(name: String, email: String) -> Result<User, AppError> {
    // Try to create user in database
    match insert_user_into_db(name, email).await {
        Ok(user) => Ok(user),
        Err(e) => Err(AppError::DbError(e.to_string())),
    }
}


#[server]
pub async fn insert_user_into_db(name: String, email: String) -> Result<User, AppError> {
    Err(AppError::DbError("Did not work".to_string()))
}

#[component]
pub fn CustomErrors() -> impl IntoView {
    view!{
        <p>"Custom errors placeholder"</p>
    }
}