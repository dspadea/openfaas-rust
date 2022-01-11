use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct RequestObject {
    name: String
}

#[derive(Serialize, Debug)]
pub struct ResponseObject {
    greeting: String,
    vars: HashMap<String, String>
}

pub async fn handle(req: RequestObject) -> ResponseObject {

    let vars: HashMap<String, String> = std::env::vars().collect();

    ResponseObject {
        greeting: format!("Hello, {}", req.name),
        vars: vars
    }
}

