use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_server_info;
pub mod get_supported_api_list;

pub async fn get_supported_api_list(
    api: &mut crate::WebApi,
) -> Result<get_supported_api_list::Response> {
    let query = format!("?key={}", api.get_key()?);
    let data = api
        .request(&format!(
            "ISteamWebAPIUtil/GetSupportedAPIList/v1/{}",
            &query
        ))
        .await?;
    return get_supported_api_list::Response::from(&data);
}

pub async fn get_server_info(api: &mut crate::WebApi) -> Result<get_server_info::Response> {
    let query = format!("?key={}", api.get_key()?);
    let data = api
        .request(&format!("ISteamWebAPIUtil/GetServerInfo/v1/{}", &query))
        .await?;
    return get_server_info::Response::from(&data);
}
