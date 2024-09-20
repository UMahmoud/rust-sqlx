use actix_web::{get, post, web::{Data, Json, Path}, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use serde_json::json;
use sqlx::{self, FromRow};
use crate::{AppState, web};

#[derive(Serialize, Deserialize, FromRow)]
struct User {
    id: Option<i32>,
    first_name: String,
    last_name: String,
}

#[derive(Serialize, FromRow)]
struct Article {
    id: i32,
    title: String,
    content: String,
    created_by: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[post("/users")]
pub async fn create_user(
    state: web::Data<AppState>,
    body: web::Json<User>
) -> impl Responder {

    match sqlx::query!("INSERT INTO users(first_name, last_name) VALUES ($1, $2) RETURNING id, first_name, last_name", 
    //id,
    body.first_name,
    body.last_name,
    )
    .fetch_one(&state.db)
    .await
   {
        Ok(user) => HttpResponse::Created().json(json!(
            {"message": "Created successfully"}
        )),
        Err(e) => {
        eprintln!("Failed to create user: {:?}", e);
        HttpResponse::InternalServerError().json("Failed to create user")},
    }
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {

    match sqlx::query_as::<_, User>("SELECT id, first_name, last_name FROM users")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("No users Found"),
    }
}

#[get("/users/{id}/articles")]
pub async fn fetch_user_articles(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();

    match sqlx::query_as::<_, Article>("SELECT id, title, content, created_by FROM articles WHERE created_by = $1")
        .bind(id)
        .fetch_all(&state.db)
        .await

    {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(_) => HttpResponse::NotFound().json("No articles found"),
    }    
}

#[post("/users/{id}/articles")]
pub async fn create_user_article(
    state: web::Data<AppState>, 
    path: web::Path<i32>, 
    body: web::Json<CreateArticleBody>
) -> impl Responder {
    let id: i32 = path.into_inner();

    match sqlx::query!("INSERT INTO articles(title, content, created_by) VALUES ($1, $2, $3) RETURNING id, title, content, created_by",
            body.title,
            body.content,
            id
    )

        .fetch_one(&state.db)
        .await

    {
        Ok(_) => HttpResponse::Ok().json(json!(
            {"message": "Created successfully"}
        )),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user article"),
    }

}