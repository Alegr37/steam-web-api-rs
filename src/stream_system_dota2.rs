use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_broadcaster_info;

pub struct StreamSystemDota2<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> StreamSystemDota2<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_broadcaster_info(
        &'a mut self,
        broadcaster_steam_id: u64,
        league_id: Option<u32>,
    ) -> Result<get_broadcaster_info::Response> {
        let query = format!(
            "?key={}&broadcaster_steam_id={}&league_id={}",
            self.api.get_key()?,
            broadcaster_steam_id,
            league_id.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!(
                "IDOTA2StreamSystem_570/GetBroadcasterInfo/v1/{}",
                &query
            ))
            .await?;
        return get_broadcaster_info::Response::from(&data);
    }
}
