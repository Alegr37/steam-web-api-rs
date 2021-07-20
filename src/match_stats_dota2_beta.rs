use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_realtime_stats;

pub struct MatchStatsDota2Beta<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> MatchStatsDota2Beta<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_realtime_stats(
        &'a mut self,
        server_steam_id: u64,
    ) -> Result<get_realtime_stats::Response> {
        let query = format!(
            "?key={}&server_steam_id={}",
            self.api.get_key()?,
            server_steam_id
        );
        let data = self
            .api
            .request(&format!(
                "IDOTA2MatchStats_205790/GetRealtimeStats/v1/{}",
                &query
            ))
            .await?;
        return get_realtime_stats::Response::from(&data);
    }
}
