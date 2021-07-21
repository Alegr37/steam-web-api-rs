use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_cm_list;
pub mod get_cs_list;
pub mod get_steam_pipe_domains;

pub async fn get_steam_pipe_domains(
    api: &mut crate::WebApi,
) -> Result<get_steam_pipe_domains::Response> {
    let query = format!("?key={}", api.get_key()?);
    let data = api
        .request(&format!(
            "ISteamDirectory/GetSteamPipeDomains/v1/{}",
            &query
        ))
        .await?;
    return get_steam_pipe_domains::Response::from(&data);
}

pub async fn get_cs_list(
    api: &mut crate::WebApi,
    cellid: u32,
    maxcount: Option<u32>,
) -> Result<get_cs_list::Response> {
    let query = format!(
        "?key={}&cellid={}&maxcount={}",
        api.get_key()?,
        cellid,
        maxcount.unwrap_or_default()
    );
    let data = api
        .request(&format!("ISteamDirectory/GetCSList/v1/{}", &query))
        .await?;
    return get_cs_list::Response::from(&data);
}

pub async fn get_cm_list(
    api: &mut crate::WebApi,
    cellid: u32,
    maxcount: Option<u32>,
) -> Result<get_cm_list::Response> {
    let query = format!(
        "?key={}&cellid={}&maxcount={}",
        api.get_key()?,
        cellid,
        maxcount.unwrap_or_default()
    );
    let data = api
        .request(&format!("ISteamDirectory/GetCMList/v1/{}", &query))
        .await?;
    return get_cm_list::Response::from(&data);
}
