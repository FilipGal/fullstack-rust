use rocket::*;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[get("/users")]
pub fn user_list() -> Json<Response> {
    Json(Response::ok("List of users"))
}

#[post("/users")]
pub fn new_user() -> String {
    "Creation of new user".to_string()
}

#[get("/users/<id>")]
pub fn info_user(id: String) -> String {
    format!("Info for user {}", id)
}

#[put("/users/<id>")]
pub fn update_user(id: String) -> String {
    format!("Update info for user {}", id)
}

#[delete("/users/<id>")]
pub fn delete_user(id: String) -> String {
    format!("Delete user {}", id)
}

#[derive(Deserialize, Serialize)]
pub struct Response {
    status: i32,
    message: String,
}

impl Response {
    fn ok(msg: &str) -> Self {
        Response {
            status: 200,
            message: msg.to_string(),
        }
    }

    fn err(msg: &str) -> Self {
        Response {
            status: 404,
            message: msg.to_string(),
        }
    }
}
