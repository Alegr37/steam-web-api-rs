use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod viewer_heartbeat;

pub async fn viewer_heartbeat(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    sessionid: u64,
    token: u64,
    stream: Option<i32>,
) -> Result<viewer_heartbeat::Response> {
    let query = format!(
        "?key={}&steamid={}&sessionid={}&token={}&stream={}",
        api.get_key()?,
        steamid,
        sessionid,
        token,
        stream.unwrap_or_default()
    );
    let data = api
        .request(&format!("ISteamBroadcast/ViewerHeartbeat/v1/{}", &query))
        .await?;
    return viewer_heartbeat::Response::from(&data);
}
