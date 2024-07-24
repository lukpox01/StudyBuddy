use actix_web::{web, HttpResponse, post, Responder};
use validator::{Validate, ValidationError};
use crate::models::auth::RegisterInput;

// 1. Register User
//    POST /api/auth/register
//    Input:  { "username": string, "email": string, "password": string }
//    Output: { "id": string, "username": string, "email": string, "token": string }

#[post("/register")]
async fn register(input: web::Json<RegisterInput>) -> impl Responder {
    let input = input.into_inner();
    match input.validate() {
        Ok(_) => HttpResponse::Ok().body("Valid input"),
        Err(e) => {
            HttpResponse::BadRequest().json(e)
        }
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
