use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_global_achievement_percentages_for_app;
pub mod get_global_stats_for_game;
pub mod get_number_of_current_players;
pub mod get_player_achievements;
pub mod get_schema_for_game;
pub mod get_user_stats_for_game;

pub async fn get_user_stats_for_game(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    appid: u32,
) -> Result<get_user_stats_for_game::Response> {
    let query = format!(
        "?key={}&steamid={}&appid={}",
        api.get_key()?,
        steamid,
        appid
    );
    let data = api
        .request(&format!(
            "ISteamUserStats/GetUserStatsForGame/v2/{}",
            &query
        ))
        .await?;
    return get_user_stats_for_game::Response::from(&data);
}

pub async fn get_schema_for_game(
    api: &mut crate::WebApi,
    appid: u32,
    l: Option<&str>,
) -> Result<get_schema_for_game::Response> {
    let query = format!(
        "?key={}&appid={}&l={}",
        api.get_key()?,
        appid,
        l.unwrap_or_default()
    );
    let data = api
        .request(&format!("ISteamUserStats/GetSchemaForGame/v2/{}", &query))
        .await?;
    return get_schema_for_game::Response::from(&data);
}

pub async fn get_player_achievements(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    appid: u32,
    l: Option<&str>,
) -> Result<get_player_achievements::Response> {
    let query = format!(
        "?key={}&steamid={}&appid={}&l={}",
        api.get_key()?,
        steamid,
        appid,
        l.unwrap_or_default()
    );
    let data = api
        .request(&format!(
            "ISteamUserStats/GetPlayerAchievements/v1/{}",
            &query
        ))
        .await?;
    return get_player_achievements::Response::from(&data);
}

pub async fn get_number_of_current_players(
    api: &mut crate::WebApi,
    appid: u32,
) -> Result<get_number_of_current_players::Response> {
    let query = format!("?key={}&appid={}", api.get_key()?, appid);
    let data = api
        .request(&format!(
            "ISteamUserStats/GetNumberOfCurrentPlayers/v1/{}",
            &query
        ))
        .await?;
    return get_number_of_current_players::Response::from(&data);
}

pub async fn get_global_stats_for_game(
    api: &mut crate::WebApi,
    appid: u32,
    count: u32,
    name_0: &str,
    startdate: Option<u32>,
    enddate: Option<u32>,
) -> Result<get_global_stats_for_game::Response> {
    let query = format!(
        "?key={}&appid={}&count={}&name[0]={}&startdate={}&enddate={}",
        api.get_key()?,
        appid,
        count,
        name_0,
        startdate.unwrap_or_default(),
        enddate.unwrap_or_default()
    );
    let data = api
        .request(&format!(
            "ISteamUserStats/GetGlobalStatsForGame/v1/{}",
            &query
        ))
        .await?;
    return get_global_stats_for_game::Response::from(&data);
}

pub async fn get_global_achievement_percentages_for_app(
    api: &mut crate::WebApi,
    gameid: u64,
) -> Result<get_global_achievement_percentages_for_app::Response> {
    let query = format!("?key={}&gameid={}", api.get_key()?, gameid);
    let data = api
        .request(&format!(
            "ISteamUserStats/GetGlobalAchievementPercentagesForApp/v2/{}",
            &query
        ))
        .await?;
    return get_global_achievement_percentages_for_app::Response::from(&data);
}
