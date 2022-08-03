#[cfg(feature = "binary")]
use lambda_runtime::{service_fn, Error};
#[cfg(feature = "binary")]
use serde_json;
#[cfg(feature = "binary")]
use hegel::http;

#[cfg(feature = "binary")]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[cfg(feature = "binary")]
async fn func(req: http::Event) -> Result<http::Response, Error> {
    // print to log
    println!("{}", serde_json::to_string(&req.payload).unwrap());
    let js = serde_json::to_string(&req.payload);
    if js.is_err() {return Ok(http::Response::new_status(500).body_text("Can not encode as json".to_string()))}
    return Ok(http::Response::new_json(js.unwrap()))
}