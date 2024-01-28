use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub struct FullUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserBody<T> {
    pub user: T,
}

#[derive(Deserialize)]
pub struct LoginUserPayload {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Default, PartialEq, Eq)]
#[serde(default)]
pub struct UpdateUserPayload {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ReturnUserPayload {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}
