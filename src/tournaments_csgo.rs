// THIS FILE HAS BEEN AUTOGENERATED AND MAY BE REDACTED (DELETE THIS LINE IF YOU WANT TO SAVE CHANGES)

use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_tournament_fantasy_lineup;
pub mod get_tournament_items;
pub mod get_tournament_layout;
pub mod get_tournament_predictions;
pub mod upload_tournament_fantasy_lineup;
pub mod upload_tournament_predictions;

pub async fn upload_tournament_predictions(
    api: &mut crate::WebApi,
    event: u32,
    steamid: crate::SteamId,
    steamidkey: &str,
    sectionid: u32,
    groupid: u32,
    index: u32,
    pickid: u32,
    itemid: u64,
) -> Result<upload_tournament_predictions::Response> {
    let query = format!("?key={}&event={}&steamid={}&steamidkey={}&sectionid={}&groupid={}&index={}&pickid={}&itemid={}", api.get_key()?, event, steamid, steamidkey, sectionid, groupid, index, pickid, itemid);
    let data = api
        .request(&format!(
            "ICSGOTournaments_730/UploadTournamentPredictions/v1/{}",
            &query
        ))
        .await?;
    return upload_tournament_predictions::Response::from(&data);
}

pub async fn upload_tournament_fantasy_lineup(
    api: &mut crate::WebApi,
    event: u32,
    steamid: crate::SteamId,
    steamidkey: &str,
    sectionid: u32,
    pickid0: u32,
    itemid0: u64,
    pickid1: u32,
    itemid1: u64,
    pickid2: u32,
    itemid2: u64,
    pickid3: u32,
    itemid3: u64,
    pickid4: u32,
    itemid4: u64,
) -> Result<upload_tournament_fantasy_lineup::Response> {
    let query = format!("?key={}&event={}&steamid={}&steamidkey={}&sectionid={}&pickid0={}&itemid0={}&pickid1={}&itemid1={}&pickid2={}&itemid2={}&pickid3={}&itemid3={}&pickid4={}&itemid4={}", api.get_key()?, event, steamid, steamidkey, sectionid, pickid0, itemid0, pickid1, itemid1, pickid2, itemid2, pickid3, itemid3, pickid4, itemid4);
    let data = api
        .request(&format!(
            "ICSGOTournaments_730/UploadTournamentFantasyLineup/v1/{}",
            &query
        ))
        .await?;
    return upload_tournament_fantasy_lineup::Response::from(&data);
}

pub async fn get_tournament_predictions(
    api: &mut crate::WebApi,
    event: u32,
    steamid: crate::SteamId,
    steamidkey: &str,
) -> Result<get_tournament_predictions::Response> {
    let query = format!(
        "?key={}&event={}&steamid={}&steamidkey={}",
        api.get_key()?,
        event,
        steamid,
        steamidkey
    );
    let data = api
        .request(&format!(
            "ICSGOTournaments_730/GetTournamentPredictions/v1/{}",
            &query
        ))
        .await?;
    return get_tournament_predictions::Response::from(&data);
}

pub async fn get_tournament_layout(
    api: &mut crate::WebApi,
    event: u32,
) -> Result<get_tournament_layout::Response> {
    let query = format!("?key={}&event={}", api.get_key()?, event);
    let data = api
        .request(&format!(
            "ICSGOTournaments_730/GetTournamentLayout/v1/{}",
            &query
        ))
        .await?;
    return get_tournament_layout::Response::from(&data);
}

pub async fn get_tournament_items(
    api: &mut crate::WebApi,
    event: u32,
    steamid: crate::SteamId,
    steamidkey: &str,
) -> Result<get_tournament_items::Response> {
    let query = format!(
        "?key={}&event={}&steamid={}&steamidkey={}",
        api.get_key()?,
        event,
        steamid,
        steamidkey
    );
    let data = api
        .request(&format!(
            "ICSGOTournaments_730/GetTournamentItems/v1/{}",
            &query
        ))
        .await?;
    return get_tournament_items::Response::from(&data);
}

pub async fn get_tournament_fantasy_lineup(
    api: &mut crate::WebApi,
    event: u32,
    steamid: crate::SteamId,
    steamidkey: &str,
) -> Result<get_tournament_fantasy_lineup::Response> {
    let query = format!(
        "?key={}&event={}&steamid={}&steamidkey={}",
        api.get_key()?,
        event,
        steamid,
        steamidkey
    );
    let data = api
        .request(&format!(
            "ICSGOTournaments_730/GetTournamentFantasyLineup/v1/{}",
            &query
        ))
        .await?;
    return get_tournament_fantasy_lineup::Response::from(&data);
}
