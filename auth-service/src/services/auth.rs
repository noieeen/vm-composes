use crate::{
    db::Db,
    utils::{hash_password, verify_password},
};
use axum::{Json, extract::State, http::StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::warn;

// #[derive(Error, Debug)]
// pub enum AuthError {
//     #[error("Database error: {0}")]
//     DatabaseError(#[from] Error),
//     #[error("Invalid credentials")]
//     InvalidCredentials,
//     #[error("Email already exists")]
//     EmailExists,
//     #[error("Username already exists")]
//     UsernameExists,
//     #[error("JWT error: {0}")]
//     JwtError(#[from] jsonwebtoken::errors::Error),
//     #[error("Password hashing error")]
//     HashingError,
// }

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub message: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub async fn login(
    State(db): State<Db>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user = db
        .get_user_by_email(&payload.email)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !verify_password(&payload.password, &user.password).map_err(|_| StatusCode::UNAUTHORIZED)? {
        warn!("Unauthorized login attempt for user: {}", &payload.email);

        return Err(StatusCode::UNAUTHORIZED);
    }

    let claims = Claims {
        sub: user.id.to_string(),
        exp: chrono::Utc::now().timestamp() as usize + 3600,
    };

    let key = EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes());
    let token =
        encode(&Header::default(), &claims, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse { token }))
}

pub async fn register(
    State(db): State<Db>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<RegisterResponse>, StatusCode> {
    let hashed = hash_password(&payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = db
        .create_user(&payload.email, &hashed)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(RegisterResponse {
        message: format!("User {} registered successfully", user.email),
        email: user.email,
    }))
}

pub async fn get_users(State(db): State<Db>) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let users = db
        .get_users()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_responses: Vec<UserResponse> = users
        .into_iter()
        .map(|user| UserResponse { email: user.email })
        .collect();

    Ok(Json(user_responses))
}

// pub async fn get_user_handler(db: Result<Db, sqlx::Error>) -> Result<Json<UserResponse>, (StatusCode, String)> {
//     let db = db.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;

//     let user = db.get_user_by_email("test@example.com")
//         .await
//         .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Query failed: {}", e)))?;

//     Ok(Json(UserResponse { email: user.email }));
// }
