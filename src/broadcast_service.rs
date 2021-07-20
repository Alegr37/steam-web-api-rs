use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod post_game_data_frame_rtmp;

pub struct BroadcastService<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> BroadcastService<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn post_game_data_frame_rtmp(
        &'a mut self,
        appid: u32,
        steamid: crate::SteamId,
        rtmp_token: &str,
        frame_data: &str,
    ) -> Result<post_game_data_frame_rtmp::Response> {
        let query = format!(
            "?key={}&appid={}&steamid={}&rtmp_token={}&frame_data={}",
            self.api.get_key()?,
            appid,
            steamid,
            rtmp_token,
            frame_data
        );
        let data = self
            .api
            .request(&format!(
                "IBroadcastService/PostGameDataFrameRTMP/v1/{}",
                &query
            ))
            .await?;
        return post_game_data_frame_rtmp::Response::from(&data);
    }
}
