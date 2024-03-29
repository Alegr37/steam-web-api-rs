// THIS FILE HAS BEEN AUTOGENERATED AND MAY BE REDACTED (DELETE THIS LINE IF YOU WANT TO SAVE CHANGES)

use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod claim_badge_reward;
pub mod get_steam_id_for_badge_id;
pub mod set_steam_account_purchased;
pub mod steam_account_valid_for_badge_type;

pub async fn steam_account_valid_for_badge_type(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    valid_badge_type1: u32,
    valid_badge_type2: u32,
    valid_badge_type3: u32,
) -> Result<steam_account_valid_for_badge_type::Response> {
    let query = format!(
        "?key={}&steamid={}&ValidBadgeType1={}&ValidBadgeType2={}&ValidBadgeType3={}",
        api.get_key()?,
        steamid,
        valid_badge_type1,
        valid_badge_type2,
        valid_badge_type3
    );
    let data = api
        .request(&format!(
            "IDOTA2Ticket_570/SteamAccountValidForBadgeType/v1/{}",
            &query
        ))
        .await?;
    return steam_account_valid_for_badge_type::Response::from(&data);
}

pub async fn set_steam_account_purchased(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    badge_type: u32,
) -> Result<set_steam_account_purchased::Response> {
    let query = format!(
        "?key={}&steamid={}&BadgeType={}",
        api.get_key()?,
        steamid,
        badge_type
    );
    let data = api
        .request(&format!(
            "IDOTA2Ticket_570/SetSteamAccountPurchased/v1/{}",
            &query
        ))
        .await?;
    return set_steam_account_purchased::Response::from(&data);
}

pub async fn get_steam_id_for_badge_id(
    api: &mut crate::WebApi,
    badge_id: &str,
) -> Result<get_steam_id_for_badge_id::Response> {
    let query = format!("?key={}&BadgeID={}", api.get_key()?, badge_id);
    let data = api
        .request(&format!(
            "IDOTA2Ticket_570/GetSteamIDForBadgeID/v1/{}",
            &query
        ))
        .await?;
    return get_steam_id_for_badge_id::Response::from(&data);
}

pub async fn claim_badge_reward(
    api: &mut crate::WebApi,
    badge_id: &str,
    valid_badge_type1: u32,
    valid_badge_type2: u32,
    valid_badge_type3: u32,
) -> Result<claim_badge_reward::Response> {
    let query = format!(
        "?key={}&BadgeID={}&ValidBadgeType1={}&ValidBadgeType2={}&ValidBadgeType3={}",
        api.get_key()?,
        badge_id,
        valid_badge_type1,
        valid_badge_type2,
        valid_badge_type3
    );
    let data = api
        .request(&format!("IDOTA2Ticket_570/ClaimBadgeReward/v1/{}", &query))
        .await?;
    return claim_badge_reward::Response::from(&data);
}
