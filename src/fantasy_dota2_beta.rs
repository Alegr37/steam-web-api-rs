use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_fantasy_player_stats;
pub mod get_player_official_info;
pub mod get_pro_player_list;

pub async fn get_pro_player_list(api: &mut crate::WebApi) -> Result<get_pro_player_list::Response> {
    let query = format!("?key={}", api.get_key()?);
    let data = api
        .request(&format!(
            "IDOTA2Fantasy_205790/GetProPlayerList/v1/{}",
            &query
        ))
        .await?;
    return get_pro_player_list::Response::from(&data);
}

pub async fn get_player_official_info(
    api: &mut crate::WebApi,
    accountid: u32,
) -> Result<get_player_official_info::Response> {
    let query = format!("?key={}&accountid={}", api.get_key()?, accountid);
    let data = api
        .request(&format!(
            "IDOTA2Fantasy_205790/GetPlayerOfficialInfo/v1/{}",
            &query
        ))
        .await?;
    return get_player_official_info::Response::from(&data);
}

pub async fn get_fantasy_player_stats(
    api: &mut crate::WebApi,
    fantasy_league_id: u32,
    start_time: Option<u32>,
    end_time: Option<u32>,
    match_id: Option<u64>,
    series_id: Option<u32>,
    player_account_id: Option<u32>,
) -> Result<get_fantasy_player_stats::Response> {
    let query = format!("?key={}&FantasyLeagueID={}&StartTime={}&EndTime={}&MatchID={}&SeriesID={}&PlayerAccountID={}", api.get_key()?, fantasy_league_id, start_time.unwrap_or_default(), end_time.unwrap_or_default(), match_id.unwrap_or_default(), series_id.unwrap_or_default(), player_account_id.unwrap_or_default());
    let data = api
        .request(&format!(
            "IDOTA2Fantasy_205790/GetFantasyPlayerStats/v1/{}",
            &query
        ))
        .await?;
    return get_fantasy_player_stats::Response::from(&data);
}
