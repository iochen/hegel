use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestContextSimple {
    pub account_id: String,
    pub api_id: String,
    pub domain_name: String,
    pub http: Http,
    pub stage: String,
    pub time_epoch: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestContext {
    pub account_id: String,
    pub api_id: String,
    pub authentication: Option<Authentication>,
    pub authorizer: Option<Authorizer>,
    pub domain_name: String,
    pub domain_prefix: String,
    pub http: Http,
    pub request_id: String,
    pub route_key: String,
    pub stage: String,
    pub time: String,
    pub time_epoch: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authentication {
    pub client_cert: ClientCert,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCert {
    pub client_cert_pem: String,
    #[serde(rename = "subjectDN")]
    pub subject_dn: String,
    #[serde(rename = "issuerDN")]
    pub issuer_dn: String,
    pub serial_number: String,
    pub validity: Validity,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authorizer {
    pub lambda: Option<HashMap<String, String>>,
    pub jwt: Option<HashMap<String,String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Validity {
    pub not_before: String,
    pub not_after: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Http {
    pub method: String,
    pub path: String,
    pub protocol: String,
    pub source_ip: String,
    pub user_agent: String,
}