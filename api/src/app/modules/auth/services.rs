use actix_web::{web, error, Responder, Result};
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
use crate::app::models::{CommonResponse, NoResponseData};

#[derive(Deserialize, Serialize)]
pub struct LoginBody {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginJwtPayload {
    id: String
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct FoundUser {
    id: uuid::Uuid,
    password: Option<String>
}

#[post("/auth/login")]
pub async fn login(
    state: web::Data<AppState>,
    info: web::Json<LoginBody>
) -> Result<impl Responder> {
    let found_user = sqlx::query_as!(
        FoundUser,
        r#"SELECT id, password FROM users WHERE username = $1"#,
        &info.username
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

        Ok(web::Json(CommonResponse {
            status: 200,
            data: Some(token),
            message: Some("Login successful".to_string())
        }))
    } else {
        Err(actix_web::error::ErrorUnauthorized("Invalid credentials"))
    }
}

#[derive(Deserialize)]
pub struct RegisterBody {
    firstname: String,
    lastname: String,
    username: String,
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

    match sqlx::query!(
        r#"
            INSERT INTO users (firstname, lastname, username, email, password)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        &info.firstname,
        &info.lastname,
        &info.username,
        &info.email,
        &hashed_password.to_string()
    )
        .fetch_optional(&state.pool)
        .await
        .is_ok() {
            true => Ok(web::Json(CommonResponse {
                status: 200,
                data: NoResponseData,
                message: Some("User created successfully".to_string())
            })),
            _ => Err(error::ErrorBadRequest("Failed to create user"))
        }
}
