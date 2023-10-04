use models::{AddTaskToUserDto, GetTaskByIdQuery, GetTasksByUserIdQuery, InsertTaskDto};
use orientdb_client::common::types::error::OrientError;
use orientdb_client::common::types::OResult;
use orientdb_client::sync::session::OSession;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;

pub async fn insert_task(task: InsertTaskDto, session: Arc<Mutex<OSession>>)-> Result<impl warp::Reply, Infallible> {
    let mut session = session.lock().unwrap();
    let results : Vec<Result<OResult, OrientError>> = session
    .command("INSERT INTO Customers (email, firstname, lastname) VALUES (:email, :firstname, :lastname)")
    .named(&[("email", &cust.email.unwrap()), ("firstname", &cust.firstname.unwrap()), ("lastname", &cust.lastname.unwrap())])
    .run()
    .unwrap()
    .collect();

}
