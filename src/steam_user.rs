use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_friend_list;
pub mod get_player_bans;
pub mod get_player_summaries;
pub mod get_user_group_list;
pub mod resolve_vanity_url;

pub async fn resolve_vanity_url(
    api: &mut crate::WebApi,
    vanityurl: &str,
    url_type: Option<i32>,
) -> Result<resolve_vanity_url::Response> {
    let query = format!(
        "?key={}&vanityurl={}&url_type={}",
        api.get_key()?,
        vanityurl,
        url_type.unwrap_or_default()
    );
    let data = api
        .request(&format!("ISteamUser/ResolveVanityURL/v1/{}", &query))
        .await?;
    return resolve_vanity_url::Response::from(&data);
}

pub async fn get_user_group_list(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
) -> Result<get_user_group_list::Response> {
    let query = format!("?key={}&steamid={}", api.get_key()?, steamid);
    let data = api
        .request(&format!("ISteamUser/GetUserGroupList/v1/{}", &query))
        .await?;
    return get_user_group_list::Response::from(&data);
}

pub async fn get_player_summaries(
    api: &mut crate::WebApi,
    steamids: Vec<crate::SteamId>,
) -> Result<get_player_summaries::Response> {
    let query = format!(
        "?key={}&steamids={}",
        api.get_key()?,
        steamids.iter().join(",")
    );
    let data = api
        .request(&format!("ISteamUser/GetPlayerSummaries/v2/{}", &query))
        .await?;
    return get_player_summaries::Response::from(&data);
}

pub async fn get_player_bans(
    api: &mut crate::WebApi,
    steamids: Vec<crate::SteamId>,
) -> Result<get_player_bans::Response> {
    let query = format!(
        "?key={}&steamids={}",
        api.get_key()?,
        steamids.iter().join(",")
    );
    let data = api
        .request(&format!("ISteamUser/GetPlayerBans/v1/{}", &query))
        .await?;
    return get_player_bans::Response::from(&data);
}

pub async fn get_friend_list(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    relationship: Option<&str>,
) -> Result<get_friend_list::Response> {
    let query = format!(
        "?key={}&steamid={}&relationship={}",
        api.get_key()?,
        steamid,
        relationship.unwrap_or_default()
    );
    let data = api
        .request(&format!("ISteamUser/GetFriendList/v1/{}", &query))
        .await?;
    return get_friend_list::Response::from(&data);
}
