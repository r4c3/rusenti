use crate::api::{
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
    response::Html,
    extract::State,
    routing::{get, post},
    Json,
    Router
};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/", get(|| async {
            Html("<h1>success</h1>")
        }))
        // .route("/api/user", post(create_user))
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
