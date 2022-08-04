use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::common;

#[cfg(feature = "chrono")]
use chrono::{Utc, TimeZone, DateTime};

/// **lambda_runtime** service simplified payload type
/// Used for building API Gateway Lambda Authorizers for HTTP APIs
///
/// example:
/// ```
/// use hegel::auth;
/// use lambda_runtime::{Error, LambdaEvent};
///
/// async fn handler(req: LambdaEvent<auth::req::RequestSimple>) -> Result<auth::Response, Error> {
///     //...
///     Ok(auth::Response::new_nc(true))
/// }
/// ```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestSimple {
    pub raw_path: String,
    pub cookies: Option<Vec<String>>,
    pub headers: HashMap<String, String>,
    pub query_string_parameters: Option<HashMap<String,String>>,
    pub request_context: common::RequestContextSimple,
    pub path_parameters: Option<HashMap<String, String>>,
    pub stage_variables: Option<HashMap<String, String>>,
}

/// **lambda_runtime** service payload type
/// Used for building API Gateway Lambda Authorizers for HTTP APIs
///
/// example:
/// ```
/// use hegel::auth;
/// use lambda_runtime::{Error, LambdaEvent};
///
/// async fn handler(req: LambdaEvent<auth::req::Request>) -> Result<auth::Response, Error> {
///     //...
///     Ok(auth::Response::new_nc(true))
/// }
/// ```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub version: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub route_arn: String,
    pub identity_source: Vec<String>,
    pub route_key: String,
    pub raw_path: String,
    pub raw_query_string: String,
    pub cookies: Option<Vec<String>>,
    pub headers: HashMap<String, String>,
    pub query_string_parameters: Option<HashMap<String,String>>,
    pub request_context: common::RequestContext,
    pub path_parameters: Option<HashMap<String, String>>,
    pub stage_variables: Option<HashMap<String, String>>,
}


impl RequestSimple {
    /// Get HTTP request path
    ///
    /// example: `/foo/bar`
    pub fn path(&self) -> String {
        self.request_context.http.path.clone()
    }

    /// Get user request cookies
    pub fn cookies(&self) -> Option<HashMap<String, String>> {
        let cs = self.cookies.clone();
        if cs.is_none() {
            return None
        }
        let cs = cs.unwrap();
        let mut result = HashMap::new();
        for c in cs {
            let spl = c.split("=").collect::<Vec<&str>>();
            if spl.len() != 2 {
                continue;
            }
            result.insert(spl[0].to_string(), spl[1].to_string());
        }
        Some(result)
    }

    /// Get user request headers
    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    /// Get user request queries
    ///
    /// example:
    /// URL: `https://iochen.com/foor/bar?a=1&b=2`
    /// Result HashMap:
    /// ```text
    /// "a" -> "1"
    /// "b" -> "2"
    /// ```
    pub fn queries(&self) -> Option<HashMap<String, String>> {
        self.query_string_parameters.clone()
    }

    /// Get route params
    ///
    /// example:
    /// Route: `GET /foo/{proxy+}`
    /// Request: `GET /foo/bar`
    /// Result HashMap:
    /// ```text
    /// "proxy" -> "bar"
    /// ```
    pub fn params(&self) -> Option<HashMap<String, String>> {
        self.path_parameters.clone()
    }

    /// Get API Gateway stage
    ///
    /// example: `$default`
    pub fn stage(&self) -> String {
        self.request_context.stage.clone()
    }

    /// Get request datetime (Instant)
    pub fn time(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_millis(self.request_context.time_epoch)
    }

    /// Get request datetime with **chrono::DateTime** type output
    /// ! Remember to enable feature **chrono** before using it !
    #[cfg(feature = "chrono")]
    pub fn time_chrono(&self) -> DateTime<Utc> {
        Utc.timestamp_millis(self.request_context.time_epoch as i64)
    }

    /// Get user request method
    ///
    /// example: `GET`, `POST`, `DELETE` ...
    pub fn method(&self) -> String {
        self.request_context.http.method.clone()
    }

    /// Get user request IP
    pub fn ip(&self) -> String {
        self.request_context.http.source_ip.clone()
    }

    /// Get user request User-Agent
    pub fn ua(&self) -> String {
        self.request_context.http.user_agent.clone()
    }

    /// Get user request HTTP protocol
    ///
    /// example: `HTTP/1.1`
    pub fn protocol(&self) -> String {
        self.request_context.http.protocol.clone()
    }
}



impl Request {
    /// Get HTTP request path
    ///
    /// example: `/foo/bar`
    pub fn path(&self) -> String {
        self.request_context.http.path.clone()
    }

    /// Get user request cookies
    pub fn cookies(&self) -> Option<HashMap<String, String>> {
        let cs = self.cookies.clone();
        if cs.is_none() {
            return None
        }
        let cs = cs.unwrap();
        let mut result = HashMap::new();
        for c in cs {
            let spl = c.split("=").collect::<Vec<&str>>();
            if spl.len() != 2 {
                continue;
            }
            result.insert(spl[0].to_string(), spl[1].to_string());
        }
        Some(result)
    }

    /// Get user request headers
    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    /// Get user request queries
    ///
    /// example:
    /// URL: `https://iochen.com/foor/bar?a=1&b=2`
    /// Result HashMap:
    /// ```text
    /// "a" -> "1"
    /// "b" -> "2"
    /// ```
    pub fn queries(&self) -> Option<HashMap<String, String>> {
        self.query_string_parameters.clone()
    }

    /// Get route params
    ///
    /// example:
    /// Route: `GET /foo/{proxy+}`
    /// Request: `GET /foo/bar`
    /// Result HashMap:
    /// ```text
    /// "proxy" -> "bar"
    /// ```
    pub fn params(&self) -> Option<HashMap<String, String>> {
        self.path_parameters.clone()
    }

    /// Get API Gateway stage
    ///
    /// example: `$default`
    pub fn stage(&self) -> String {
        self.request_context.stage.clone()
    }

    /// Get request datetime (Instant)
    pub fn time(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_millis(self.request_context.time_epoch)
    }

    /// Get request datetime with **chrono::DateTime** type output
    /// ! Remember to enable feature **chrono** before using it !
    #[cfg(feature = "chrono")]
    pub fn time_chrono(&self) -> DateTime<Utc> {
        Utc.timestamp_millis(self.request_context.time_epoch as i64)
    }

    /// Get user request method
    ///
    /// example: `GET`, `POST`, `DELETE` ...
    pub fn method(&self) -> String {
        self.request_context.http.method.clone()
    }

    /// Get user request IP
    pub fn ip(&self) -> String {
        self.request_context.http.source_ip.clone()
    }

    /// Get user request User-Agent
    pub fn ua(&self) -> String {
        self.request_context.http.user_agent.clone()
    }

    /// Get user request HTTP protocol
    ///
    /// example: `HTTP/1.1`
    pub fn protocol(&self) -> String {
        self.request_context.http.protocol.clone()
    }
}