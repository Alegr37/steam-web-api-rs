use crate::web_api;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub struct ISteamWebAPIUtilApi<'a> {
    pub web_api: &'a mut web_api::WebApi,
}

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

impl<'a> ISteamWebAPIUtilApi<'a> {
    pub fn new(web_api: &'a mut web_api::WebApi) -> Self {
        return Self { web_api };
    }

    /// Get supported by api interfaces.
    pub async fn get_supported_api_list(&'a mut self) -> Result<Vec<ApiInterface>> {
        let data = self
            .web_api
            .request("ISteamWebAPIUtil/GetSupportedAPIList/v0001/")
            .await?;
        let j: SupportedApiListResponse = serde_json::from_slice(&data)?;
        Ok(j.apilist.interfaces)
    }
}
