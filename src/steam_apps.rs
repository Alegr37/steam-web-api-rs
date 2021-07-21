use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_app_list;
pub mod get_sdr_config;
pub mod get_servers_at_address;
pub mod up_to_date_check;

pub async fn up_to_date_check(
    api: &mut crate::WebApi,
    appid: u32,
    version: u32,
) -> Result<up_to_date_check::Response> {
    let query = format!(
        "?key={}&appid={}&version={}",
        api.get_key()?,
        appid,
        version
    );
    let data = api
        .request(&format!("ISteamApps/UpToDateCheck/v1/{}", &query))
        .await?;
    return up_to_date_check::Response::from(&data);
}

pub async fn get_servers_at_address(
    api: &mut crate::WebApi,
    addr: &str,
) -> Result<get_servers_at_address::Response> {
    let query = format!("?key={}&addr={}", api.get_key()?, addr);
    let data = api
        .request(&format!("ISteamApps/GetServersAtAddress/v1/{}", &query))
        .await?;
    return get_servers_at_address::Response::from(&data);
}

pub async fn get_sdr_config(
    api: &mut crate::WebApi,
    appid: u32,
) -> Result<get_sdr_config::Response> {
    let query = format!("?key={}&appid={}", api.get_key()?, appid);
    let data = api
        .request(&format!("ISteamApps/GetSDRConfig/v1/{}", &query))
        .await?;
    return get_sdr_config::Response::from(&data);
}

pub async fn get_app_list(api: &mut crate::WebApi) -> Result<get_app_list::Response> {
    let query = format!("?key={}", api.get_key()?);
    let data = api
        .request(&format!("ISteamApps/GetAppList/v2/{}", &query))
        .await?;
    return get_app_list::Response::from(&data);
}
