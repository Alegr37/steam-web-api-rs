use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_client_update_hosts;
pub mod get_depot_patch_info;
pub mod get_servers_for_steam_pipe;

pub struct ContentServerDirectoryService<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> ContentServerDirectoryService<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_servers_for_steam_pipe(
        &'a mut self,
        cell_id: u32,
        max_servers: Option<u32>,
        ip_override: Option<&str>,
        launcher_type: Option<i32>,
        ipv6_public: Option<&str>,
    ) -> Result<get_servers_for_steam_pipe::Response> {
        let query = format!(
            "?key={}&cell_id={}&max_servers={}&ip_override={}&launcher_type={}&ipv6_public={}",
            self.api.get_key()?,
            cell_id,
            max_servers.unwrap_or_default(),
            ip_override.unwrap_or_default(),
            launcher_type.unwrap_or_default(),
            ipv6_public.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!(
                "IContentServerDirectoryService/GetServersForSteamPipe/v1/{}",
                &query
            ))
            .await?;
        return get_servers_for_steam_pipe::Response::from(&data);
    }

    pub async fn get_depot_patch_info(
        &'a mut self,
        appid: u32,
        depotid: u32,
        source_manifestid: u64,
        target_manifestid: u64,
    ) -> Result<get_depot_patch_info::Response> {
        let query = format!(
            "?key={}&appid={}&depotid={}&source_manifestid={}&target_manifestid={}",
            self.api.get_key()?,
            appid,
            depotid,
            source_manifestid,
            target_manifestid
        );
        let data = self
            .api
            .request(&format!(
                "IContentServerDirectoryService/GetDepotPatchInfo/v1/{}",
                &query
            ))
            .await?;
        return get_depot_patch_info::Response::from(&data);
    }

    pub async fn get_client_update_hosts(
        &'a mut self,
        cached_signature: &str,
    ) -> Result<get_client_update_hosts::Response> {
        let query = format!(
            "?key={}&cached_signature={}",
            self.api.get_key()?,
            cached_signature
        );
        let data = self
            .api
            .request(&format!(
                "IContentServerDirectoryService/GetClientUpdateHosts/v1/{}",
                &query
            ))
            .await?;
        return get_client_update_hosts::Response::from(&data);
    }
}
