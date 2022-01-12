use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct RequestObject {
    name: String,
}

#[derive(Serialize, Debug)]
pub struct ResponseObject {
    greeting: String,
}

#[derive(Serialize, Debug)]
pub struct ErrorObject {
    error: String,
}


pub async fn handle(req: RequestObject) -> Result<ResponseObject, ErrorObject> {
    Ok(ResponseObject {
        greeting: format!("Hello, {}", req.name)
    })
}
