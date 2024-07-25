use crate::models::auth::RegisterInput;
use crate::models::users::User;
use crate::Database;
use actix_web::{post, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde_json::json;
use surrealdb::sql::{Datetime, Thing, Value};
use uuid::Uuid;
use validator::{Validate, ValidationError};

// 1. Register User
//    POST /api/auth/register
//    Input:  { "username": string, "email": string, "password": string }
//    Output: { "id": string, "username": string, "email": string, "token": string }

#[post("/register")]
async fn register(input: web::Json<RegisterInput>, db: web::Data<Database>) -> impl Responder {
    let input = input.into_inner();
    match input.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(e),
    }

    let password = bcrypt::hash(input.password.clone(), bcrypt::DEFAULT_COST).unwrap();

    match db
        .insert_user(User {
            id: Thing::from(("user", Uuid::new_v4().to_string().as_str())),
            username: input.username,
            email: input.email,
            password_hash: password,
            created_at: Datetime(Utc::now()),
            last_login: None,
            status: "active".to_string(),
        })
        .await
    {
        Ok(v) => HttpResponse::Ok().json(json!({ "id": v })),
        Err(e) => HttpResponse::InternalServerError().json(json!(e)),
    }
}

//
// 2. Login User
//    POST /api/auth/login
//    Input:  { "email": string, "password": string }
//    Output: { "id": string, "username": string, "accessToken": string, "refreshToken": string, "expiresIn": number }
//
// 3. Logout User
//    POST /api/auth/logout
//    Input:  No body, requires valid access token in Authorization header
//    Output: { "message": string }
//
//
// 6. Change Password
//    POST /api/auth/change-password
//    Input:  { "currentPassword": string, "newPassword": string }, requires valid access token in Authorization header
//    Output: { "message": string }
//
// 7. Forgot Password
//    POST /api/auth/forgot-password
//    Input:  { "email": string }
//    Output: { "message": string }
//
// 8. Reset Password
//    POST /api/auth/reset-password
//    Input:  { "token": string, "newPassword": string }
//    Output: { "message": string }
//
// 9. Refresh Token
//    POST /api/auth/refresh-token
//    Input:  { "refreshToken": string }
//    Output: { "accessToken": string, "refreshToken": string, "expiresIn": number }
