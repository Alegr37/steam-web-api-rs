use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod post_game_data_frame_rtmp;

pub async fn post_game_data_frame_rtmp(
    api: &mut crate::WebApi,
    appid: u32,
    steamid: crate::SteamId,
    rtmp_token: &str,
    frame_data: &str,
) -> Result<post_game_data_frame_rtmp::Response> {
    let query = format!(
        "?key={}&appid={}&steamid={}&rtmp_token={}&frame_data={}",
        api.get_key()?,
        appid,
        steamid,
        rtmp_token,
        frame_data
    );
    let data = api
        .request(&format!(
            "IBroadcastService/PostGameDataFrameRTMP/v1/{}",
            &query
        ))
        .await?;
    return post_game_data_frame_rtmp::Response::from(&data);
}
