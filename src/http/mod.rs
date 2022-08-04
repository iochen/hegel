pub mod req;
pub mod resp;
pub mod utils;

pub use resp::Response;
pub use req::{Request, RequestSimple};

use lambda_runtime::LambdaEvent;

/// Event for lambda_runtime service function
/// Used for building API Gateway Lambda proxy integrations for HTTP APIs
///
/// example:
/// ```rust
/// use lambda_runtime::{service_fn, Error};
/// use hegel::http;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let func = service_fn(func);
///     lambda_runtime::run(func).await?;
///     Ok(())
/// }
///
/// async fn func(req: http::Event) -> Result<http::Response, Error> {
///     // ...
///     Ok(http::Response::new_status(200))
/// }
/// ```
///
pub type Event = LambdaEvent<Request>;