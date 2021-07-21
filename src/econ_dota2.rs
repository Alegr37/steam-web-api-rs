use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_event_stats_for_account;
pub mod get_game_items;
pub mod get_heroes;
pub mod get_item_creators;
pub mod get_item_workshop_published_file_i_ds;
pub mod get_rarities;
pub mod get_tournament_prize_pool;

pub async fn get_tournament_prize_pool(
    api: &mut crate::WebApi,
    leagueid: Option<u32>,
) -> Result<get_tournament_prize_pool::Response> {
    let query = format!(
        "?key={}&leagueid={}",
        api.get_key()?,
        leagueid.unwrap_or_default()
    );
    let data = api
        .request(&format!(
            "IEconDOTA2_570/GetTournamentPrizePool/v1/{}",
            &query
        ))
        .await?;
    return get_tournament_prize_pool::Response::from(&data);
}

pub async fn get_rarities(
    api: &mut crate::WebApi,
    language: Option<&str>,
) -> Result<get_rarities::Response> {
    let query = format!(
        "?key={}&language={}",
        api.get_key()?,
        language.unwrap_or_default()
    );
    let data = api
        .request(&format!("IEconDOTA2_570/GetRarities/v1/{}", &query))
        .await?;
    return get_rarities::Response::from(&data);
}

pub async fn get_item_workshop_published_file_i_ds(
    api: &mut crate::WebApi,
    itemdef: u32,
) -> Result<get_item_workshop_published_file_i_ds::Response> {
    let query = format!("?key={}&itemdef={}", api.get_key()?, itemdef);
    let data = api
        .request(&format!(
            "IEconDOTA2_570/GetItemWorkshopPublishedFileIDs/v1/{}",
            &query
        ))
        .await?;
    return get_item_workshop_published_file_i_ds::Response::from(&data);
}

pub async fn get_item_creators(
    api: &mut crate::WebApi,
    itemdef: u32,
) -> Result<get_item_creators::Response> {
    let query = format!("?key={}&itemdef={}", api.get_key()?, itemdef);
    let data = api
        .request(&format!("IEconDOTA2_570/GetItemCreators/v1/{}", &query))
        .await?;
    return get_item_creators::Response::from(&data);
}

pub async fn get_heroes(
    api: &mut crate::WebApi,
    language: Option<&str>,
    itemizedonly: Option<bool>,
) -> Result<get_heroes::Response> {
    let query = format!(
        "?key={}&language={}&itemizedonly={}",
        api.get_key()?,
        language.unwrap_or_default(),
        itemizedonly.unwrap_or_default()
    );
    let data = api
        .request(&format!("IEconDOTA2_570/GetHeroes/v1/{}", &query))
        .await?;
    return get_heroes::Response::from(&data);
}

pub async fn get_game_items(
    api: &mut crate::WebApi,
    language: Option<&str>,
) -> Result<get_game_items::Response> {
    let query = format!(
        "?key={}&language={}",
        api.get_key()?,
        language.unwrap_or_default()
    );
    let data = api
        .request(&format!("IEconDOTA2_570/GetGameItems/v1/{}", &query))
        .await?;
    return get_game_items::Response::from(&data);
}

pub async fn get_event_stats_for_account(
    api: &mut crate::WebApi,
    eventid: u32,
    accountid: u32,
    language: Option<&str>,
) -> Result<get_event_stats_for_account::Response> {
    let query = format!(
        "?key={}&eventid={}&accountid={}&language={}",
        api.get_key()?,
        eventid,
        accountid,
        language.unwrap_or_default()
    );
    let data = api
        .request(&format!(
            "IEconDOTA2_570/GetEventStatsForAccount/v1/{}",
            &query
        ))
        .await?;
    return get_event_stats_for_account::Response::from(&data);
}
