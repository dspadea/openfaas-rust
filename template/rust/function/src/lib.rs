use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct RequestObject {
    name: String
}

#[derive(Serialize, Debug)]
pub struct ResponseObject {
    greeting: String
}

pub async fn handle(req: RequestObject) -> ResponseObject {

    ResponseObject {
        greeting: format!("Hello, {}", req.name)
    }
}
