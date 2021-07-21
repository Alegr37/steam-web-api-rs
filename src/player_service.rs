use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_badges;
pub mod get_community_badge_progress;
pub mod get_owned_games;
pub mod get_recently_played_games;
pub mod get_steam_level;
pub mod is_playing_shared_game;
pub mod record_offline_playtime;

pub async fn record_offline_playtime(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    ticket: &str,
    play_sessions: String,
) -> Result<record_offline_playtime::Response> {
    let query = format!(
        "?key={}&steamid={}&ticket={}&play_sessions={}",
        api.get_key()?,
        steamid,
        ticket,
        play_sessions
    );
    let data = api
        .request(&format!(
            "IPlayerService/RecordOfflinePlaytime/v1/{}",
            &query
        ))
        .await?;
    return record_offline_playtime::Response::from(&data);
}

pub async fn is_playing_shared_game(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    appid_playing: u32,
) -> Result<is_playing_shared_game::Response> {
    let query = format!(
        "?key={}&steamid={}&appid_playing={}",
        api.get_key()?,
        steamid,
        appid_playing
    );
    let data = api
        .request(&format!("IPlayerService/IsPlayingSharedGame/v1/{}", &query))
        .await?;
    return is_playing_shared_game::Response::from(&data);
}

pub async fn get_steam_level(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
) -> Result<get_steam_level::Response> {
    let query = format!("?key={}&steamid={}", api.get_key()?, steamid);
    let data = api
        .request(&format!("IPlayerService/GetSteamLevel/v1/{}", &query))
        .await?;
    return get_steam_level::Response::from(&data);
}

pub async fn get_recently_played_games(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    count: u32,
) -> Result<get_recently_played_games::Response> {
    let query = format!(
        "?key={}&steamid={}&count={}",
        api.get_key()?,
        steamid,
        count
    );
    let data = api
        .request(&format!(
            "IPlayerService/GetRecentlyPlayedGames/v1/{}",
            &query
        ))
        .await?;
    return get_recently_played_games::Response::from(&data);
}

pub async fn get_owned_games(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    include_appinfo: bool,
    include_played_free_games: bool,
    appids_filter: u32,
    include_free_sub: bool,
    skip_unvetted_apps: Option<bool>,
) -> Result<get_owned_games::Response> {
    let query = format!("?key={}&steamid={}&include_appinfo={}&include_played_free_games={}&appids_filter={}&include_free_sub={}&skip_unvetted_apps={}", api.get_key()?, steamid, include_appinfo, include_played_free_games, appids_filter, include_free_sub, skip_unvetted_apps.unwrap_or_default());
    let data = api
        .request(&format!("IPlayerService/GetOwnedGames/v1/{}", &query))
        .await?;
    return get_owned_games::Response::from(&data);
}

pub async fn get_community_badge_progress(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    badgeid: i32,
) -> Result<get_community_badge_progress::Response> {
    let query = format!(
        "?key={}&steamid={}&badgeid={}",
        api.get_key()?,
        steamid,
        badgeid
    );
    let data = api
        .request(&format!(
            "IPlayerService/GetCommunityBadgeProgress/v1/{}",
            &query
        ))
        .await?;
    return get_community_badge_progress::Response::from(&data);
}

pub async fn get_badges(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
) -> Result<get_badges::Response> {
    let query = format!("?key={}&steamid={}", api.get_key()?, steamid);
    let data = api
        .request(&format!("IPlayerService/GetBadges/v1/{}", &query))
        .await?;
    return get_badges::Response::from(&data);
}
