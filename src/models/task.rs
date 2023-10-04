use orientdb_client::common::types::error::OrientError;
use orientdb_client::common::types::OResult;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};

#[derive(Serialize, EnumString, Display, Eq, PartialEq)]
pub enum TaskState {
    NotStarted,
    InProgress,
    Completed,
    Paused,
    Failed
}

#[derive(Serialize)]
pub struct Task {
    pub rid: String,
    pub user_rid: String,
    pub task_type: String,
    pub title: String,
    pub description: String,
    pub state: TaskState
}

#[derive(Serialize)]
pub struct InsertTaskDto {
    pub user_rid: String,
    pub task_type: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTaskByIdQuery{
    pub rid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTasksByUserIdQuery{
    pub user_rid: String,
}