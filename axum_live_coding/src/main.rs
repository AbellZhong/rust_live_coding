use std::net::SocketAddr;

use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode, request::Parts},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router, Server, TypedHeader,
};
use jsonwebtoken as JWT;
use serde::{Deserialize, Serialize};
use JWT::Validation;

//eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6Inpob25nc2hlbmNoYW9AZm94bWFpbC5jb20iLCJwYXNzd29yZCI6IjEyMyJ9.RUELQ_KXHrMPdSd_ClJjlT2u5CjFXg8phTCd2qgYmXk

const SECRET: &[u8] = b"zhongshenchao";

#[derive(Deserialize, Serialize, Debug)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims{
    id: i32,
    name: String,
    exp: usize
}

#[derive(Debug, Deserialize, Serialize)]
enum HttpError {
    AuthorError = 401,
    InternalError = 500,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let (code, msg) = match self {
            HttpError::AuthorError => (
                axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                "Author Invalid",
            ),
            HttpError::InternalError => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "SERVER ERROR",
            ),
        };

        (code, msg).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // You can either call them directly...
        let TypedHeader(Authorization(bearer)) =
        TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
            .await
            .map_err(|_| HttpError::AuthorError)?;

        // ... or use `extract` / `extract_with_state` from `RequestExt` / `RequestPartsExt`
        let key = JWT::DecodingKey::from_secret(SECRET);

        let token = JWT::decode(bearer.token(), &key, &Validation::default())
            .map_err(|_| HttpError::AuthorError)?;

        Ok(token.claims)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct CreateTodo {
    title: String,
}

async fn index_handler() -> Html<&'static str> {
    Html("Hello")
}

async fn todo_handler() -> Json<Vec<Todo>> {
    Json(vec![
        Todo {
            id: 1,
            title: "Hello".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "World".to_string(),
            completed: true,
        },
    ])
}


async fn create_todo_handler(claims: Claims, Json(todo): Json<CreateTodo>) -> Json<String> {
    Json("Hello".to_string())
}

async fn login_handler(Json(user_info): Json<LoginRequest>) -> Json<LoginResponse> {
    let claims = Claims{
        id: 1,
        name: "zhongshenchao".to_string(),
        exp: get_epoch() + 14 * 24 * 60 * 60,
    };

    let key = JWT::EncodingKey::from_secret(SECRET);

    let token = JWT::encode(&JWT::Header::default(), &claims, &key).unwrap();

    Json(LoginResponse { token })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/todo", get(todo_handler).post(create_todo_handler))
        .route("/login", post(login_handler));

    let addr = SocketAddr::from(([192, 168, 71, 133], 8080));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


fn get_epoch() -> usize{
    use std::time::SystemTime;

    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}