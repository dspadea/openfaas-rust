extern crate handler;

use std::io;
use std::panic;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ErrorMessage {
    error: ErrorValue,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorValue {
    message: String,
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let req = match serde_json::from_reader(handle) {
        Ok(r) => r,
        Err(e) => {
            print!("{}",
                   serde_json::to_string(&ErrorMessage {
                       error: ErrorValue {
                           message: format!("{:#?}", e)
                       }
                   }).unwrap());
            return Ok(());
        }
    };

    let block_result = panic::catch_unwind(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        return rt.block_on(async {
            handler::handle(req).await
        });
    });

    let result = match block_result {
        Ok(response_obj) => {
            match response_obj {
                Ok(r) => serde_json::to_string(&r).unwrap(),
                Err(e) => serde_json::to_string(&e).unwrap(),
            }
        }
        Err(e) => {
            serde_json::to_string(&ErrorMessage {
                error: ErrorValue {
                    message: format!("{:#?}", e)
                }
            }).unwrap()
        }
    };

    println!("{}", result);

    Ok(())
}
