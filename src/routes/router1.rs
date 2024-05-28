use rocket::serde::json::Json;
use rocket::http::Status;
use crate::models::MyData;
use crate::controllers::{get_data_by_id,create_data,get_data_by_query_id as query_get_handler};

#[get("/get1")]
pub fn get1() -> Json<MyData> {
    Json(MyData { id: 1, name: "Router 1 Get 1".to_string() })
}

#[get("/get2")]
pub fn get2() -> Json<MyData> {
    Json(MyData { id: 2, name: "Router 1 Get 2".to_string() })
}

#[post("/post", format = "json", data = "<data>")]
pub fn post(data: Json<MyData>) -> Json<MyData> {
    Json(MyData { id: data.id, name: format!("Router 1 Post: {}", data.name) })
}

#[get("/get-by-id/<id>")]
pub fn get_by_id(id: u32) -> Result<Json<MyData>, Status> {
    let data = get_data_by_id(id)?;
    Ok(Json(data))
}

#[get("/get-by-query-id?<id>")]
pub fn get_by_query_id(id: Option<u32>) -> Result<Json<MyData>, Status> {
    let data = query_get_handler(id)?;
    Ok(Json(data))
}

#[post("/create", format = "json", data = "<data>")]
pub fn create(data: Json<MyData>) -> Result<Json<MyData>, Status> {
    let data = create_data(data.into_inner())?;
    Ok(Json(data))
}