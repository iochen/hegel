use crate::common;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[cfg(feature = "chrono")]
use chrono::{DateTime, TimeZone, Utc};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestSimple {
    pub raw_path: String,
    pub cookies: Option<Vec<String>>,
    pub headers: HashMap<String, String>,
    pub query_string_parameters: Option<HashMap<String,String>>,
    pub request_context: common::RequestContext,
    pub body: Option<String>,
    pub path_parameters: Option<HashMap<String, String>>,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64encoded: bool,
    pub stage_variables: Option<HashMap<String, String>>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub version: String,
    pub route_key: String,
    pub raw_path: String,
    pub raw_query_string: String,
    pub cookies: Option<Vec<String>>,
    pub headers: HashMap<String, String>,
    pub query_string_parameters: Option<HashMap<String,String>>,
    pub request_context: common::RequestContext,
    pub body: Option<String>,
    pub path_parameters: Option<HashMap<String, String>>,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64encoded: bool,
    pub stage_variables: Option<HashMap<String, String>>,
}

pub enum ParseBodyError {
    Base64DecodeError(base64::DecodeError),
    FromUtf8Error(std::string::FromUtf8Error)
}

impl RequestSimple {
    pub fn body(&self) -> Result<Option<String>, ParseBodyError> {
        if self.body.is_none() {
            return Ok(None)
        }
        return if !self.is_base64encoded {
            Ok(self.body.clone())
        } else {
            let result = base64::decode(self.body.clone().unwrap());
            if result.is_err() { return Err(ParseBodyError::Base64DecodeError(result.unwrap_err()))}
            let result = String::from_utf8(result.unwrap());
            if result.is_err() {return Err(ParseBodyError::FromUtf8Error(result.unwrap_err()))}
            Ok(Some(result.unwrap()))
        }
    }

    pub fn body_binary(&self) -> Result<Option<Vec<u8>>, base64::DecodeError> {
        if self.body.is_none() {
            return Ok(None)
        }
        return if !self.is_base64encoded {
            Ok(self.body.clone().map(|b|b.as_bytes().to_vec()))
        } else {
            Ok(Some(base64::decode(self.body.clone().unwrap())?.as_slice().to_vec()))
        }
    }

    pub fn path(&self) -> String {
        self.request_context.http.path.clone()
    }

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

    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    pub fn queries(&self) -> Option<HashMap<String, String>> {
        self.query_string_parameters.clone()
    }

    pub fn params(&self) -> Option<HashMap<String, String>> {
        self.path_parameters.clone()
    }

    pub fn stage(&self) -> String {
        self.request_context.stage.clone()
    }

    pub fn time(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_millis(self.request_context.time_epoch)
    }

    #[cfg(feature = "chrono")]
    pub fn time_chrono(&self) -> DateTime<Utc> {
        Utc.timestamp_millis(self.request_context.time_epoch as i64)
    }

    pub fn method(&self) -> String {
        self.request_context.http.method.clone()
    }

    pub fn ip(&self) -> String {
        self.request_context.http.source_ip.clone()
    }

    pub fn ua(&self) -> String {
        self.request_context.http.user_agent.clone()
    }

    pub fn protocol(&self) -> String {
        self.request_context.http.protocol.clone()
    }
}

impl Request {
    pub fn body(&self) -> Result<Option<String>, ParseBodyError> {
        if self.body.is_none() {
            return Ok(None)
        }
        return if !self.is_base64encoded {
            Ok(self.body.clone())
        } else {
            let result = base64::decode(self.body.clone().unwrap());
            if result.is_err() { return Err(ParseBodyError::Base64DecodeError(result.unwrap_err()))}
            let result = String::from_utf8(result.unwrap());
            if result.is_err() {return Err(ParseBodyError::FromUtf8Error(result.unwrap_err()))}
            Ok(Some(result.unwrap()))
        }
    }

    pub fn body_binary(&self) -> Result<Option<Vec<u8>>, base64::DecodeError> {
        if self.body.is_none() {
            return Ok(None)
        }
        return if !self.is_base64encoded {
            Ok(self.body.clone().map(|b|b.as_bytes().to_vec()))
        } else {
            Ok(Some(base64::decode(self.body.clone().unwrap())?.as_slice().to_vec()))
        }
    }

    pub fn path(&self) -> String {
        self.request_context.http.path.clone()
    }

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

    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    pub fn queries(&self) -> Option<HashMap<String, String>> {
        self.query_string_parameters.clone()
    }

    pub fn params(&self) -> Option<HashMap<String, String>> {
        self.path_parameters.clone()
    }

    pub fn stage(&self) -> String {
        self.request_context.stage.clone()
    }

    pub fn time(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_millis(self.request_context.time_epoch)
    }

    #[cfg(feature = "chrono")]
    pub fn time_chrono(&self) -> DateTime<Utc> {
        Utc.timestamp_millis(self.request_context.time_epoch as i64)
    }

    pub fn method(&self) -> String {
        self.request_context.http.method.clone()
    }

    pub fn ip(&self) -> String {
        self.request_context.http.source_ip.clone()
    }

    pub fn ua(&self) -> String {
        self.request_context.http.user_agent.clone()
    }

    pub fn protocol(&self) -> String {
        self.request_context.http.protocol.clone()
    }
}






