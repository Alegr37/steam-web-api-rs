use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_realtime_stats;

pub async fn get_realtime_stats(
    api: &mut crate::WebApi,
    server_steam_id: u64,
) -> Result<get_realtime_stats::Response> {
    let query = format!(
        "?key={}&server_steam_id={}",
        api.get_key()?,
        server_steam_id
    );
    let data = api
        .request(&format!(
            "IDOTA2MatchStats_570/GetRealtimeStats/v1/{}",
            &query
        ))
        .await?;
    return get_realtime_stats::Response::from(&data);
}
