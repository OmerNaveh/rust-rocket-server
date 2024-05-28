use rocket::serde::json::Json;
use crate::models::MyData;

#[get("/get1")]
pub fn get1() -> Json<MyData> {
    Json(MyData { id: 1, name: "Router 2 Get 1".to_string() })
}

#[get("/get2")]
pub fn get2() -> Json<MyData> {
    Json(MyData { id: 2, name: "Router 2 Get 2".to_string() })
}

#[post("/post", format = "json", data = "<data>")]
pub fn post(data: Json<MyData>) -> Json<MyData> {
    Json(MyData { id: data.id, name: format!("Router 2 Post: {}", data.name) })
}