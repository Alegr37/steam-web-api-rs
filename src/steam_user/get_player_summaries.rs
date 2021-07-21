use crate::common::*;
use anyhow::Result;
use chrono::serde::{ts_seconds, ts_seconds_option};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum PersonaState {
    // (Also set when the profile is Private)
    Offline = 0,
    Online = 1,
    Busy = 2,
    Away = 3,
    Snooze = 4,
    LookingToTrade = 5,
    LookingToPlay = 6,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ProfileState {
    Unconfigured = 0,
    Configured = 1,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum CommunityVisibilityState {
    Private = 1,
    FriendsOnly = 2,
    FriendsOfFriends = 3,
    UsersOnly = 4,
    Public = 5,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerSummaries {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub steamid: SteamId,
    #[serde(rename = "communityvisibilitystate")]
    pub community_visibility_state: CommunityVisibilityState,
    #[serde(rename = "profilestate")]
    pub profile_state: ProfileState,
    #[serde(rename = "personaname")]
    pub persona_name: String,
    #[serde(rename = "profileurl")]
    pub profile_url: String,
    #[serde(rename = "avatar")]
    pub avatar: String,
    #[serde(rename = "avatarmedium")]
    pub avatar_medium: String,
    #[serde(rename = "avatarfull")]
    pub avatar_full: String,
    #[serde(rename = "avatarhash")]
    pub avatar_hash: String,
    #[serde(rename = "lastlogoff", with = "ts_seconds")]
    pub last_logoff: DateTime<Utc>,
    #[serde(rename = "personastate")]
    pub persona_state: PersonaState,
    #[serde(rename = "commentpermission")]
    pub comment_permission: Option<usize>,
    #[serde(rename = "realname")]
    pub real_name: Option<String>,
    #[serde(rename = "primaryclanid")]
    pub primary_clan_id: Option<String>,
    #[serde(rename = "timecreated", with = "ts_seconds_option")]
    pub time_created: Option<DateTime<Utc>>,
    #[serde(rename = "personastateflags")]
    pub persona_state_flags: usize,
    #[serde(rename = "loccountrycode")]
    pub loc_country_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerSummariesInnerResponse {
    players: Vec<PlayerSummaries>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerSummariesResponse {
    response: PlayerSummariesInnerResponse,
}

pub struct Response {
    pub players: Vec<PlayerSummaries>,
}

impl Response {
    pub fn from(data: &[u8]) -> Result<Self> {
        let j: PlayerSummariesResponse = serde_json::from_slice(&data)?;
        Ok(Response {
            players: j.response.players,
        })
    }
}
