#[cfg(feature = "binary")]
use lambda_runtime::{service_fn, Error};
#[cfg(feature = "binary")]
use serde_json;

use hegel::auth;

#[tokio::main]
#[cfg(feature = "binary")]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[cfg(feature = "binary")]
async fn func(req: auth::AuthEvent) -> Result<auth::Response, Error> {
    println!("{}", serde_json::to_string(&req.payload).unwrap());
    Ok(auth::Response::new_nc(true))
}