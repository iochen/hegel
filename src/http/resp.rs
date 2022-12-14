use std::cmp::min;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// **lambda_runtime** service function return payload type
/// Used for building API Gateway Lambda proxy integrations for HTTP APIs
///
/// example:
/// ```
/// use hegel::http;
/// use lambda_runtime::Error;
///
/// async fn handler(req: http::Event) -> Result<http::Response, Error> {
///     //...
///     Ok(http::Response::new_status(200))
/// }
/// ```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(rename = "isBase64Encoded")]
    pub is_base64encoded: bool,
    pub status_code: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl Response {
    /// return a file as Response
    pub fn new_file(b: Vec<u8>) -> Response {
        let mut headers = HashMap::new();
        let mime = infer::get(b.get(0..min(31, b.len() - 1)).unwrap());
        if mime.is_some() {
            headers.insert("Content-Type".to_string(), mime.unwrap().to_string());
        } else {
            headers.insert("Content-Type".to_string(), "application/octet-stream".to_string());
        }
        Response {
            is_base64encoded: true,
            status_code: 200,
            body: base64::encode(b),
            headers
        }
    }

    /// return html(UTF-8) as Response
    pub fn new_html(b: String) -> Response {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
        Response {
            is_base64encoded: false,
            status_code: 200,
            body: b,
            headers
        }
    }

    /// return json as Response
    pub fn new_json(b: String) -> Response {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        Response {
            is_base64encoded: false,
            status_code: 200,
            body: b,
            headers
        }
    }

    /// return text(UTF-8) as Response
    pub fn new_text(b: String) -> Response {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain; charset=utf-8".to_string());
        Response {
            is_base64encoded: false,
            status_code: 200,
            body: b,
            headers
        }
    }

    /// return a HTTP status as Response
    pub fn new_status(s: u16) -> Response {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain; charset=utf-8".to_string());
        Response {
            is_base64encoded: false,
            status_code: s,
            body: super::utils::status_code::meaning(s).unwrap_or("An unknown error occurred").to_string(),
            headers
        }
    }

    /// return a Response with provided header added
    /// like s struct builder
    pub fn header(mut self, k: String, v: String) -> Response {
        self.headers.insert(k,v);
        self
    }

    /// return a Response with provided status code added
    /// like s struct builder
    pub fn status_code(mut self, s:u16) -> Response {
        self.status_code = s;
        self
    }

    /// return a Response with provided text body added
    /// like s struct builder
    pub fn body_text(mut self, b: String) -> Response {
        self.headers.insert("Content-Type".to_string(), "text/plain; charset=utf-8".to_string());
        self.body = b;
        self.is_base64encoded = false;
        self
    }

    /// return a Response with provided json body added
    /// like s struct builder
    pub fn body_json(mut self, b: String) -> Response {
        self.headers.insert("Content-Type".to_string(), "application/json".to_string());
        self.body = b;
        self.is_base64encoded = false;
        self
    }

    /// return a Response with provided html body added
    /// like s struct builder
    pub fn body_html(mut self, b: String) -> Response {
        self.headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
        self.body = b;
        self.is_base64encoded = false;
        self
    }

    /// return a Response with provided file body added
    /// like s struct builder
    pub fn body_file(mut self, b: Vec<u8>) -> Response {
        let mime = infer::get(b.get(0..min(31, b.len() - 1)).unwrap());
        if mime.is_some() {
            self.headers.insert("Content-Type".to_string(), mime.unwrap().to_string());
        } else {
            self.headers.insert("Content-Type".to_string(), "application/octet-stream".to_string());
        }
        self.body = base64::encode(b);
        self.is_base64encoded = true;
        self
    }

    /// return a Response with provided body added
    /// like s struct builder
    pub fn body(mut self, body: String, base64_encoded: bool, mime_type: String) -> Response {
        self.headers.insert("Content-Type".to_string(), mime_type);
        self.body = body;
        self.is_base64encoded = base64_encoded;
        self
    }
}


