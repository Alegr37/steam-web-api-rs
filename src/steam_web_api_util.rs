use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_server_info;
pub mod get_supported_api_list;

pub struct SteamWebApiUtil<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> SteamWebApiUtil<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_supported_api_list(&'a mut self) -> Result<get_supported_api_list::Response> {
        let query = format!("?key={}", self.api.get_key()?);
        let data = self
            .api
            .request(&format!(
                "ISteamWebAPIUtil/GetSupportedAPIList/v1/{}",
                &query
            ))
            .await?;
        return get_supported_api_list::Response::from(&data);
    }

    pub async fn get_server_info(&'a mut self) -> Result<get_server_info::Response> {
        let query = format!("?key={}", self.api.get_key()?);
        let data = self
            .api
            .request(&format!("ISteamWebAPIUtil/GetServerInfo/v1/{}", &query))
            .await?;
        return get_server_info::Response::from(&data);
    }
}
