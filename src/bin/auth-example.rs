use std::collections::HashMap;
#[cfg(feature = "binary")]
use lambda_runtime::{service_fn, Error};

#[cfg(feature = "binary")]
use serde_json;

#[cfg(feature = "binary")]
use hegel::auth;

#[cfg(feature = "binary")]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[cfg(feature = "binary")]
async fn func(req: auth::Event) -> Result<auth::Response, Error> {
    // print to log
    println!("{}", serde_json::to_string(&req.payload).unwrap());
    return match req.payload.path().as_str() {
        "/" => Ok(auth::Response::new_nc(true)),
        "/pass" => Ok(auth::Response::new_nc(true)),
        "/pass_with_context" => {
            let mut context = HashMap::new();
            context.insert("type".to_string(), "sudo".to_string());
            context.insert("user_type".to_string(), "admin".to_string());
            Ok(auth::Response::new(true, context))
        },
        "/deny" => Ok(auth::Response::new_nc(false)),
        "/deny_with_context" => {
            let mut context = HashMap::new();
            context.insert("type".to_string(), "failed".to_string());
            context.insert("user_type".to_string(), "visitor".to_string());
            Ok(auth::Response::new(true, context))
        }
        _ => Ok(auth::Response::new_nc(true))
    }
}