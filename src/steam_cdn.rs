use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod set_client_filters;
pub mod set_performance_stats;

pub struct SteamCdn<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> SteamCdn<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn set_performance_stats(
        &'a mut self,
        cdnname: &str,
        mbps_sent: Option<u32>,
        mbps_recv: Option<u32>,
        cpu_percent: Option<u32>,
        cache_hit_percent: Option<u32>,
    ) -> Result<set_performance_stats::Response> {
        let query = format!(
            "?key={}&cdnname={}&mbps_sent={}&mbps_recv={}&cpu_percent={}&cache_hit_percent={}",
            self.api.get_key()?,
            cdnname,
            mbps_sent.unwrap_or_default(),
            mbps_recv.unwrap_or_default(),
            cpu_percent.unwrap_or_default(),
            cache_hit_percent.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("ISteamCDN/SetPerformanceStats/v1/{}", &query))
            .await?;
        return set_performance_stats::Response::from(&data);
    }

    pub async fn set_client_filters(
        &'a mut self,
        cdnname: &str,
        allowedipblocks: Option<&str>,
        allowedasns: Option<&str>,
        allowedipcountries: Option<&str>,
    ) -> Result<set_client_filters::Response> {
        let query = format!(
            "?key={}&cdnname={}&allowedipblocks={}&allowedasns={}&allowedipcountries={}",
            self.api.get_key()?,
            cdnname,
            allowedipblocks.unwrap_or_default(),
            allowedasns.unwrap_or_default(),
            allowedipcountries.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("ISteamCDN/SetClientFilters/v1/{}", &query))
            .await?;
        return set_client_filters::Response::from(&data);
    }
}
