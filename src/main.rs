#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HealthStatus {
    status: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Post {
    id: i32,
    user_id: i32,
    title: String,
    body: String,
}

#[get("/health")]
fn health() -> Json<HealthStatus> {
    Json(HealthStatus { status: "ok".to_string(), version: "0.1.0".to_string() })
}

#[get("/health/ready")]
fn ready() -> Json<HealthStatus> {
    Json(HealthStatus { status: "ready".to_string(), version: "0.1.0".to_string() })
}

#[get("/users")]
fn list_users() -> Json<Vec<User>> {
    Json(vec![
        User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() },
        User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() },
    ])
}

#[get("/users/<id>")]
fn get_user(id: i32) -> Json<User> {
    Json(User { id, name: "Sample User".to_string(), email: "user@example.com".to_string() })
}

#[post("/users", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    let mut new_user = user.into_inner();
    new_user.id = 1;
    Json(new_user)
}

#[put("/users/<id>", data = "<user>")]
fn update_user(id: i32, user: Json<User>) -> Json<User> {
    let mut updated = user.into_inner();
    updated.id = id;
    Json(updated)
}

#[delete("/users/<id>")]
fn delete_user(id: i32) -> rocket::http::Status {
    let _ = id;
    rocket::http::Status::NoContent
}

#[get("/users/<user_id>/posts")]
fn get_user_posts(user_id: i32) -> Json<Vec<Post>> {
    Json(vec![Post { id: 1, user_id, title: "User Post".to_string(), body: "Content".to_string() }])
}

#[get("/posts")]
fn list_posts() -> Json<Vec<Post>> {
    Json(vec![
        Post { id: 1, user_id: 1, title: "First Post".to_string(), body: "Hello world".to_string() },
        Post { id: 2, user_id: 1, title: "Second Post".to_string(), body: "Another post".to_string() },
    ])
}

#[get("/posts/<id>")]
fn get_post(id: i32) -> Json<Post> {
    Json(Post { id, user_id: 1, title: "Sample Post".to_string(), body: "Post body".to_string() })
}

#[post("/posts", data = "<post>")]
fn create_post(post: Json<Post>) -> Json<Post> {
    let mut new_post = post.into_inner();
    new_post.id = 1;
    Json(new_post)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        health, ready,
        list_users, get_user, create_user, update_user, delete_user, get_user_posts,
        list_posts, get_post, create_post
    ])
}
