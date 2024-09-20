use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use dotenvy::dotenv;
// use sqlx::{postgres::PgPoolOptions, Pool, postgres};
use sqlx::{PgPool, Pool, postgres::PgPoolOptions};
use std::env;

mod service;
use service::{create_user_article, fetch_user_articles, fetch_users, create_user};

pub struct AppState {
    db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    // let pool = PgPool::connect(&database_url).await.expect("Failed to create pool");
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    let app_state = web::Data::new(AppState { db: pool });

    HttpServer::new(move || {
        App::new()
            // .app_data(Data::new(AppState {db: pool.clone() }))
            .app_data(app_state.clone())
            .service(create_user)
            .service(fetch_users)
            .service(fetch_user_articles)
            .service(create_user_article)
    })
    // .bind(("127.0.0.1", 8080))?
    .bind("127.0.0.1:8080")?
    .run()
    .await
}