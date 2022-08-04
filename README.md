# Hegel
AWS **H**TTP API **G**at**e**way Payload for **L**ambda   

## Installation
```toml
[dependencies]
hegel = "0.1.0"
```

## Document
[DOC.RS](https://docs.rs/hegel)

## Introduction
Hegel provides the lightest AWS HTTP API Gateway Payload for Lambda
    
It's recommended to use hegel with [lambda_runtime](https://github.com/awslabs/aws-lambda-rust-runtime)
    
Hegel has two publicly accessible modules:    
`hegel::auth` and `hegel::http`   
### hegel::auth
This module is used for building API Gateway Lambda Authorizers for HTTP APIs   
The payloads are all designed for format 2.0   

example code:
```rust
use std::collections::HashMap;
use lambda_runtime::{service_fn, Error};
use serde_json;
use hegel::auth;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

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
```
The code is available under folder `src/bin/auth-example.rs`
To avoid the heavy dependency `tokio` as default, remember to add `--features binary` param when building the binary in this crate

### hegel::http
This module is used for building API Gateway Lambda proxy integrations for HTTP APIs   
The payloads are all designed for format 2.0   

example:
```rust
use lambda_runtime::{service_fn, Error};
use serde_json;
use hegel::http;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(req: http::Event) -> Result<http::Response, Error> {
    // print to log
    println!("{}", serde_json::to_string(&req.payload).unwrap());
    let js = serde_json::to_string(&req.payload);
    if js.is_err() {return Ok(http::Response::new_status(500).body_text("Can not encode as json".to_string()))}
    return Ok(http::Response::new_json(js.unwrap()))
}
```
The code is available under folder `src/bin/http-echo.rs`
To avoid the heavy dependency `tokio` as default, remember to add `--features binary` param when building the binary in this crate

## Optional features
### chrono
Enable it when you want to get user request datetime in `chrono::DateTime` type
### binary
Pass `--features binary` to cargo when you want to build or check codes under folder `src/bin/`   
   
example: 
```shell
$ cd ${the path to this repo}
$ cargo check --features binary
$ cargo lambda build --release --features binary
$ cargo lambda build --release --arm64 --features binary
```



## LICENSE
MIT LICENSE