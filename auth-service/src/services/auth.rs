use crate::{db::Db, utils::verify_password};
use axum::{Json, extract::State, http::StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::env;

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
    pub email: String
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
    let user = db
        .create_user(&payload.email, &payload.password)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(RegisterResponse {
        message: format!("User {} registered successfully", user.email),
        email: user.email,
    }))
}
