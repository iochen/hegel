use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// **lambda_runtime** service function return payload type
/// Used for building API Gateway Lambda Authorizers for HTTP APIs
///
/// example:
/// ```
/// use hegel::auth;
/// use lambda_runtime::Error;
///
/// async fn handler(req: auth::Event) -> Result<auth::Response, Error> {
///     //...
///     Ok(auth::Response::new_nc(true))
/// }
/// ```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub is_authorized: bool,
    pub context: HashMap<String,String>,
}

impl Response {
    /// new **Response** with context
    ///
    /// example:
    /// ```rust
    /// use std::collections::HashMap;
    /// use hegel::auth;
    ///
    /// let mut context = HashMap::new();
    /// context.insert("user_type".to_string(),"admin".to_string());
    /// auth::Response::new(true, context);
    /// ```
    ///
    /// The context can be received by the code like following:
    /// ```rust
    /// use hegel::http;
    /// use lambda_runtime::Error;
    ///
    /// async fn handler(req: http::Event) -> Result<http::Response, Error> {
    ///     let authorizer = req.payload.request_context.authorizer.clone();
    ///     if authorizer.is_none() {
    ///         return Ok(http::Response::new_text("No authorizer found".to_string()));
    ///     }
    ///     let authorizer = authorizer.unwrap().lambda;
    ///     if authorizer.is_none() {
    ///         return Ok(http::Response::new_text("Authorizer is not lambda".to_string()));
    ///     }
    ///     let authorizer = authorizer.unwrap();
    ///     let user_type = authorizer.get("user_type");
    ///     if user_type.is_none() {
    ///         return Ok(http::Response::new_text("Context user_type not found".to_string()));
    ///     }
    ///     Ok(http::Response::new_text(format!("Context user-type: {}", user_type.unwrap())))
    /// }
    /// ```
    pub fn new(is_authorized: bool, context: HashMap<String,String>) -> Response {
        Response {
            is_authorized,
            context
        }
    }

    /// new **Response** without context
    ///
    /// example:
    /// ```rust
    /// use hegel::auth;
    ///
    /// // Pass
    /// auth::Response::new_nc(true);
    ///
    /// //Deny
    /// auth::Response::new_nc(false);
    /// ```
    pub fn new_nc(is_authorized: bool) -> Response {
        Response {
            is_authorized,
            context:HashMap::new()
        }
    }
}
