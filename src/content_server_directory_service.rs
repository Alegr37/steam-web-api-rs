use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_client_update_hosts;
pub mod get_depot_patch_info;
pub mod get_servers_for_steam_pipe;

pub async fn get_servers_for_steam_pipe(
    api: &mut crate::WebApi,
    cell_id: u32,
    max_servers: Option<u32>,
    ip_override: Option<&str>,
    launcher_type: Option<i32>,
    ipv6_public: Option<&str>,
) -> Result<get_servers_for_steam_pipe::Response> {
    let query = format!(
        "?key={}&cell_id={}&max_servers={}&ip_override={}&launcher_type={}&ipv6_public={}",
        api.get_key()?,
        cell_id,
        max_servers.unwrap_or_default(),
        ip_override.unwrap_or_default(),
        launcher_type.unwrap_or_default(),
        ipv6_public.unwrap_or_default()
    );
    let data = api
        .request(&format!(
            "IContentServerDirectoryService/GetServersForSteamPipe/v1/{}",
            &query
        ))
        .await?;
    return get_servers_for_steam_pipe::Response::from(&data);
}

pub async fn get_depot_patch_info(
    api: &mut crate::WebApi,
    appid: u32,
    depotid: u32,
    source_manifestid: u64,
    target_manifestid: u64,
) -> Result<get_depot_patch_info::Response> {
    let query = format!(
        "?key={}&appid={}&depotid={}&source_manifestid={}&target_manifestid={}",
        api.get_key()?,
        appid,
        depotid,
        source_manifestid,
        target_manifestid
    );
    let data = api
        .request(&format!(
            "IContentServerDirectoryService/GetDepotPatchInfo/v1/{}",
            &query
        ))
        .await?;
    return get_depot_patch_info::Response::from(&data);
}

pub async fn get_client_update_hosts(
    api: &mut crate::WebApi,
    cached_signature: &str,
) -> Result<get_client_update_hosts::Response> {
    let query = format!(
        "?key={}&cached_signature={}",
        api.get_key()?,
        cached_signature
    );
    let data = api
        .request(&format!(
            "IContentServerDirectoryService/GetClientUpdateHosts/v1/{}",
            &query
        ))
        .await?;
    return get_client_update_hosts::Response::from(&data);
}
