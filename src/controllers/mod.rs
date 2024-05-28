use crate::models::MyData;
use rocket::http::Status;

pub fn get_data_by_id(id: u32) -> Result<MyData, Status> {
    match id {
        1 => Ok(MyData { id: 1, name: "Data 1".to_string() }),
        2 => Ok(MyData { id: 2, name: "Data 2".to_string() }),
        _ => Err(Status::NotFound),
    }
}

pub fn get_data_by_query_id(id: Option<u32>) -> Result<MyData, Status> {
    match id {
        Some(1) => Ok(MyData { id: 1, name: "Data 1".to_string() }),
        Some(2) => Ok(MyData { id: 2, name: "Data 2".to_string() }),
        _ => Err(Status::NotFound),
    }
}

pub fn create_data(data: MyData) -> Result<MyData, Status> {
    Ok(data)
}