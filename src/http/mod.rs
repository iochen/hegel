pub mod req;
pub mod resp;
pub mod utils;

pub use resp::Response;
pub use req::{Request, RequestSimple};

use lambda_runtime::LambdaEvent;

pub type Event = LambdaEvent<Request>;