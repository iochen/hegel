use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub is_authorized: bool,
    pub context: HashMap<String,String>,
}

impl Response {
    pub fn new(is_authorized: bool, context: HashMap<String,String>) -> Response {
        Response {
            is_authorized,
            context
        }
    }

    pub fn new_nc(is_authorized: bool) -> Response {
        Response {
            is_authorized,
            context:HashMap::new()
        }
    }
}
