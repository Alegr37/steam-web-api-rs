use crate::web_api;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub struct ISteamWebAPIUtilApi<'a> {
    web_api: &'a mut web_api::WebApi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupportedApiListMethodParameter {
    name: String,
    r#type: String,
    optional: bool,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupportedApiListMethod {
    name: String,
    version: usize,
    httpmethod: String,
    parameters: Vec<SupportedApiListMethodParameter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiInterface {
    name: String,
    methods: Vec<SupportedApiListMethod>,
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
