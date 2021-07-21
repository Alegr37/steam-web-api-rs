use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_game_maps_playtime;
pub mod get_game_servers_status;

pub async fn get_game_servers_status(
    api: &mut crate::WebApi,
) -> Result<get_game_servers_status::Response> {
    let query = format!("?key={}", api.get_key()?);
    let data = api
        .request(&format!(
            "ICSGOServers_730/GetGameServersStatus/v1/{}",
            &query
        ))
        .await?;
    return get_game_servers_status::Response::from(&data);
}

pub async fn get_game_maps_playtime(
    api: &mut crate::WebApi,
    interval: &str,
    gamemode: &str,
    mapgroup: &str,
) -> Result<get_game_maps_playtime::Response> {
    let query = format!(
        "?key={}&interval={}&gamemode={}&mapgroup={}",
        api.get_key()?,
        interval,
        gamemode,
        mapgroup
    );
    let data = api
        .request(&format!(
            "ICSGOServers_730/GetGameMapsPlaytime/v1/{}",
            &query
        ))
        .await?;
    return get_game_maps_playtime::Response::from(&data);
}
