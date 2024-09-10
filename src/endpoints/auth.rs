use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde_json::json;
use surrealdb::sql::{Datetime, Thing};
use uuid::Uuid;
use validator::Validate;

use crate::models::auth::{LoginInput, RegisterInput, VerifyToken};
use crate::models::users::User;
use crate::Database;
use crate::jwt::jwt::*;

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
    let id = Uuid::new_v4().to_string();
    match db
        .create_user(User {
            id: Thing::from(("user", id.clone().as_str())),
            username: input.username,
            email: input.email.clone(),
            password_hash: password,
            created_at: Datetime(Utc::now()),
            last_login: None,
            status: "unverified".to_string(),
        })
        .await
    {
        Ok(v) => {

            let secret = Uuid::new_v4().to_string();
            match db.add_secret(v[0].id.clone(), &secret).await{
                Ok(_) => (),
                Err(e) => return HttpResponse::InternalServerError().json(json!(e)),
            };

            HttpResponse::Ok().json(json!({"secret": secret}))
            //TODO send verification email
        }
        Err(e) => HttpResponse::InternalServerError().json(json!(e)),
    }
}

//
// 2. Login User
//    POST /api/auth/login
//    Input:  { "email": string, "password": string }
//    Output: { "id": string, "username": string, "accessToken": string, "refreshToken": string, "expiresIn": number }

#[post("/login")]
async fn login(input: web::Json<LoginInput>, db: web::Data<Database>) -> impl Responder {
    let input = input.into_inner();
    match input.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(e),
    }

    match db.select_user_by_email(input.email.clone().as_str()).await {
        Ok(user) => {
            if bcrypt::verify(input.password.clone(), user.password_hash.clone().as_str()).unwrap()
            {
                let secret = match db.get_secret_by_email(user.email.as_str()).await{
                    Ok(secret) => secret,
                    Err(e) => return HttpResponse::InternalServerError().json(json!(e)),
                };
                let access_token = create_access_token(user.id.to_string().as_str(), secret.token.as_bytes()).unwrap();
                HttpResponse::Ok().json(json!({"token": access_token}))
            } else {
                HttpResponse::Unauthorized()
                    .json(json!({ "message": "Invalid email or password" }))
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(json!(e)),
    }
}

// Verify Token
//    POST /api/auth/verify
//    Input:  { "token": string, email: string }
//    Output: { bool }

#[post("/verify")]
async fn verify(input: web::Json<VerifyToken>, db: web::Data<Database>) -> impl Responder {
    let input = input.into_inner();
    match input.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(e),
    }

    match db.get_secret_by_email(input.email.as_str()).await{
        Ok(secret) => {
            match verify_access_token(input.token.clone().as_str(), secret.token.as_bytes()){
                Ok(_) => HttpResponse::Ok().json(true),
                Err(_) => HttpResponse::Ok().json(false),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(json!(e)),
    }

}

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
