use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_steam_cache_node_params;
pub mod set_steam_cache_client_filters;
pub mod set_steam_cache_performance_stats;

pub struct ContentServerConfigService<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> ContentServerConfigService<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn set_steam_cache_performance_stats(
        &'a mut self,
        cache_id: u32,
        cache_key: &str,
        mbps_sent: u32,
        mbps_recv: u32,
        cpu_percent: u32,
        cache_hit_percent: u32,
        num_connected_ips: u32,
        upstream_egress_utilization: u32,
    ) -> Result<set_steam_cache_performance_stats::Response> {
        let query = format!("?key={}&cache_id={}&cache_key={}&mbps_sent={}&mbps_recv={}&cpu_percent={}&cache_hit_percent={}&num_connected_ips={}&upstream_egress_utilization={}", self.api.get_key()?, cache_id, cache_key, mbps_sent, mbps_recv, cpu_percent, cache_hit_percent, num_connected_ips, upstream_egress_utilization);
        let data = self
            .api
            .request(&format!(
                "IContentServerConfigService/SetSteamCachePerformanceStats/v1/{}",
                &query
            ))
            .await?;
        return set_steam_cache_performance_stats::Response::from(&data);
    }

    pub async fn set_steam_cache_client_filters(
        &'a mut self,
        cache_id: u32,
        cache_key: &str,
        change_notes: &str,
        allowed_ip_blocks: &str,
    ) -> Result<set_steam_cache_client_filters::Response> {
        let query = format!(
            "?key={}&cache_id={}&cache_key={}&change_notes={}&allowed_ip_blocks={}",
            self.api.get_key()?,
            cache_id,
            cache_key,
            change_notes,
            allowed_ip_blocks
        );
        let data = self
            .api
            .request(&format!(
                "IContentServerConfigService/SetSteamCacheClientFilters/v1/{}",
                &query
            ))
            .await?;
        return set_steam_cache_client_filters::Response::from(&data);
    }

    pub async fn get_steam_cache_node_params(
        &'a mut self,
        cache_id: u32,
        cache_key: &str,
    ) -> Result<get_steam_cache_node_params::Response> {
        let query = format!(
            "?key={}&cache_id={}&cache_key={}",
            self.api.get_key()?,
            cache_id,
            cache_key
        );
        let data = self
            .api
            .request(&format!(
                "IContentServerConfigService/GetSteamCacheNodeParams/v1/{}",
                &query
            ))
            .await?;
        return get_steam_cache_node_params::Response::from(&data);
    }
}
