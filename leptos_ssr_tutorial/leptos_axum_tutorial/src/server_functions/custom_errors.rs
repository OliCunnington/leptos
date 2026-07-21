use leptos::prelude::*;
use server_fn::codec::JsonEncoding;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AppError {
    ServerFnError(ServerFnErrorErr),
    DbError(String),
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
    match insert_user_into_db(&name, &email).await {
        Ok(user) => Ok(user),
        Err(e) => Err(AppError::DbError(e.to_string())),
    }
}