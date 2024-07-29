use std::env;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use db::database::Database;
use endpoints::auth;

// use jwt::JwtManager;
mod db;
mod endpoints;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let access_secret = env::var("ACCESS_KEY").expect("ACCESS_KEY must be set");
    let refresh_secret = env::var("REFRESH_KEY").expect("REFRESH_KEY must be set");

    let db = Database::new("StudyBuddy", "auth").await.unwrap();
    let db = web::Data::new(db);

    HttpServer::new(move || {
        App::new().app_data(db.clone()).service(
            web::scope("/auth")
                .service(auth::register)
                .service(auth::login),
        )
    })
    .bind("127.0.0.1:6666")?
    .run()
    .await
}
