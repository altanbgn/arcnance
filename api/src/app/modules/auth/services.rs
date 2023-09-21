use actix_web::{Responder, web, Result};
use actix_web::post;
use serde::{Deserialize, Serialize};
use scrypt::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Scrypt
};
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::AppState;
use crate::app::models::user;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginBody {
    user_name: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String
}

#[derive(Serialize)]
pub struct LoginJwtPayload {
    id: String
}

#[post("/auth/login")]
pub async fn login(
    state: web::Data<AppState>,
    info: web::Json<LoginBody>
) -> Result<impl Responder> {
    let found_user = sqlx::query_as!(
        user::User,
        "SELECT * FROM users WHERE user_name = $1",
        info.user_name
    )
        .fetch_one(&state.pool)
        .await
        .unwrap();

    let parsed_hash_password = PasswordHash::new(
        &found_user.password.as_deref().unwrap()
    ).unwrap();

    let confirmed = Scrypt.verify_password(
        info.password.as_bytes(),
        &parsed_hash_password
    ).is_ok();

    if confirmed {
        let payload = LoginJwtPayload { id: found_user.id.to_string() };
        let token = encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&state.config.secret_key.as_bytes()),
        ).unwrap().to_string();

        Ok(web::Json(LoginResponse { token }))
    } else {
        Err(actix_web::error::ErrorUnauthorized("Invalid credentials"))
    }
}

#[derive(Deserialize)]
pub struct RegisterBody {
    first_name: String,
    last_name: String,
    user_name: String,
    email: String,
    password: String
}

#[post("/auth/register")]
pub async fn register(
    state: web::Data<AppState>,
    info: web::Json<RegisterBody>
) -> Result<impl Responder> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Scrypt
        .hash_password(info.password.as_bytes(), &salt)
        .unwrap();

    let created_user = sqlx::query!(
        "INSERT INTO users (first_name, last_name, user_name, email, password)
        VALUES ($1, $2, $3, $4, $5)",
        info.first_name,
        info.last_name,
        info.user_name,
        info.email,
        hashed_password.to_string()
    )
        .execute(&state.pool)
        .await
        .is_ok();

    if created_user {
        Ok(web::Json("User created"))
    } else {
        Err(actix_web::error::ErrorBadRequest("Failed to create user"))
    }
}
