// THIS FILE HAS BEEN AUTOGENERATED AND MAY BE REDACTED (DELETE THIS LINE IF YOU WANT TO SAVE CHANGES)

use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_broadcaster_info;

pub async fn get_broadcaster_info(
    api: &mut crate::WebApi,
    broadcaster_steam_id: u64,
    league_id: Option<u32>,
) -> Result<get_broadcaster_info::Response> {
    let query = format!(
        "?key={}&broadcaster_steam_id={}&league_id={}",
        api.get_key()?,
        broadcaster_steam_id,
        league_id.unwrap_or_default()
    );
    let data = api
        .request(&format!(
            "IDOTA2StreamSystem_205790/GetBroadcasterInfo/v1/{}",
            &query
        ))
        .await?;
    return get_broadcaster_info::Response::from(&data);
}
