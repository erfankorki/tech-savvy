use super::models::Post;
use actix_web::{get, web, HttpRequest, Responder};

#[get("/posts")]
pub async fn home() -> impl Responder {
    let posts: Vec<Post> = vec![
        Post {
            title: String::from("First Post"),
        },
        Post {
            title: String::from("Second Post"),
        },
    ];
    web::Json(posts)
}
