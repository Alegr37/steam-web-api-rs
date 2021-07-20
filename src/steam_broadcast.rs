use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod viewer_heartbeat;

pub struct SteamBroadcast<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> SteamBroadcast<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn viewer_heartbeat(
        &'a mut self,
        steamid: crate::SteamId,
        sessionid: u64,
        token: u64,
        stream: Option<i32>,
    ) -> Result<viewer_heartbeat::Response> {
        let query = format!(
            "?key={}&steamid={}&sessionid={}&token={}&stream={}",
            self.api.get_key()?,
            steamid,
            sessionid,
            token,
            stream.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("ISteamBroadcast/ViewerHeartbeat/v1/{}", &query))
            .await?;
        return viewer_heartbeat::Response::from(&data);
    }
}
