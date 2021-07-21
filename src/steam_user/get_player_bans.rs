use crate::common::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerBanList {
    #[serde(
        rename = "SteamId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub steamid: SteamId,
    #[serde(rename = "CommunityBanned")]
    pub community_banned: bool,
    #[serde(rename = "VACBanned")]
    pub vac_banned: bool,
    #[serde(rename = "NumberOfVACBans")]
    pub number_of_vac_bans: usize,
    #[serde(rename = "DaysSinceLastBan")]
    pub days_since_last_ban: usize,
    #[serde(rename = "NumberOfGameBans")]
    pub number_of_game_bans: usize,
    #[serde(rename = "EconomyBan")]
    pub economy_ban: String, // TODO: enum
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub players: Vec<PlayerBanList>,
}

impl Response {
    pub fn from(data: &[u8]) -> Result<Self> {
        let j: Response = serde_json::from_slice(&data)?;
        Ok(j)
    }
}
