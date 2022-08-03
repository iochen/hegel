pub mod req;
pub mod resp;

pub use resp::Response;
pub use req::{Request, RequestSimple};

use lambda_runtime::LambdaEvent;

pub type AuthEvent = LambdaEvent<Request>;