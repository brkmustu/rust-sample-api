use orientdb_client::OrientDB;
use std::sync::{Arc, Mutex};
use warp::Filter;

#[tokio::main]
async fn main() {
    let client = OrientDB::connect(("localhost", 2424)).unwrap();
    let session = client.session("CustomerDb", "root", "rootpwd").unwrap();
    let session = Arc::new(Mutex::new(session));
    let hello_world = warp::path::end().map(|| "Hello, World at root!");
    // let routes = hello_world.or(customer_routes);
    warp::serve(hello_world).run(([127, 0, 0, 1], 8080)).await;
}

// mod filters {
//     use crate::models::{GetCustomerByIdQuery, InsertCustomerDto};
//     use orientdb_client::OSession;
//     use std::sync::{Arc, Mutex};
//     use warp::Filter;


// }

// mod handlers {}

// mod models {
//     use orientdb_client::common::types::error::OrientError;
//     use orientdb_client::common::types::OResult;
//     use serde::{Deserialize, Serialize};


// }
