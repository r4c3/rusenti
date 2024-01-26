use crate::http::{
   ApiContext,
   Result,
   models::user::{
       UserBody,
       ReturnUserPayload,
       CreateUserPayload,
   },
};
use uuid::Uuid;
use axum::{
    extract::State,
    routing::{post},
    Json,
    Router
};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/user", post(create_user))
}

#[allow(dead_code)]
async fn create_user(
    context: State<ApiContext>,
    Json(payload): Json<UserBody<CreateUserPayload>>
) -> Result<Json<UserBody<ReturnUserPayload>>> {
    Ok(Json(UserBody {
        user: ReturnUserPayload {
            id: Uuid::new_v4(),
            email: "user@skrusenti.com".to_string(),
            username: "username".to_string(),
        },
    }))
}
