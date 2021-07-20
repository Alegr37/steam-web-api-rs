use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Parameter {
    pub name: String,
    pub r#type: String,
    pub optional: bool,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Method {
    pub name: String,
    pub version: usize,
    pub httpmethod: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiInterface {
    pub name: String,
    pub methods: Vec<Method>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupportedApiListInterfaces {
    interfaces: Vec<ApiInterface>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupportedApiListResponse {
    apilist: SupportedApiListInterfaces,
}

pub struct Response {
    pub interfaces: Vec<ApiInterface>,
}

impl Response {
    pub fn from(data: &[u8]) -> Result<Self> {
        let j: SupportedApiListResponse = serde_json::from_slice(&data)?;
        Ok(Response {
            interfaces: j.apilist.interfaces,
        })
    }
}
