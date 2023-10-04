use orientdb_client::common::types::error::OrientError;
use orientdb_client::common::types::OResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    rid: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    tasks: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertUserDto {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddTaskToUserDto{
    rid: Option<String>,
    tasks: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserByIdQuery{
    pub rid: String,
}
