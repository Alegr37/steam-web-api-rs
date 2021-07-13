use crate::web_api;
use anyhow::{bail, Result};
use chrono::serde::{ts_seconds, ts_seconds_option};
use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

pub struct ISteamUserApi<'a> {
    web_api: &'a mut web_api::WebApi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Friend {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    steamid: web_api::SteamId,
    relationship: Relationship,
    #[serde(with = "ts_seconds")]
    friend_since: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Relationship {
    #[serde(rename = "friend")]
    Friend,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FriendList {
    friends: Vec<Friend>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FriendListResponse {
    friendslist: FriendList,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResolvedUrl {
    steamid: Option<String>,
    message: Option<String>,
    success: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResolvedUrlResponse {
    response: ResolvedUrl,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerBanList {
    #[serde(
        rename = "SteamId",
        deserialize_with = "deserialize_number_from_string"
    )]
    steamid: web_api::SteamId,
    #[serde(rename = "CommunityBanned")]
    community_banned: bool,
    #[serde(rename = "VACBanned")]
    vac_banned: bool,
    #[serde(rename = "NumberOfVACBans")]
    number_of_vac_bans: usize,
    #[serde(rename = "DaysSinceLastBan")]
    days_since_last_ban: usize,
    #[serde(rename = "NumberOfGameBans")]
    number_of_game_bans: usize,
    #[serde(rename = "EconomyBan")]
    economy_ban: String, // TODO: enum
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerBanListResponse {
    players: Vec<PlayerBanList>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerSummaries {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    steamid: web_api::SteamId,
    #[serde(rename = "communityvisibilitystate")]
    community_visibility_state: usize,
    #[serde(rename = "profilestate")]
    profile_state: usize,
    #[serde(rename = "personaname")]
    persona_name: String,
    #[serde(rename = "profileurl")]
    profile_url: String,
    #[serde(rename = "avatar")]
    avatar: String,
    #[serde(rename = "avatarmedium")]
    avatar_medium: String,
    #[serde(rename = "avatarfull")]
    avatar_full: String,
    #[serde(rename = "avatarhash")]
    avatar_hash: String,
    #[serde(rename = "lastlogoff", with = "ts_seconds")]
    last_logoff: DateTime<Utc>,
    #[serde(rename = "personastate")]
    persona_state: usize,
    #[serde(rename = "commentpermission")]
    comment_permission: Option<usize>,
    #[serde(rename = "realname")]
    real_name: Option<String>,
    #[serde(rename = "primaryclanid")]
    primary_clan_id: Option<String>,
    #[serde(rename = "timecreated", with = "ts_seconds_option")]
    time_created: Option<DateTime<Utc>>,
    #[serde(rename = "personastateflags")]
    persona_state_flags: usize,
    #[serde(rename = "loccountrycode")]
    loc_country_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerSummariesInnerResponse {
    players: Vec<PlayerSummaries>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerSummariesResponse {
    response: PlayerSummariesInnerResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGroup {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    guid: web_api::GroupId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGroupListInerResponse {
    success: bool,
    groups: Vec<UserGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGroupListResponse {
    response: UserGroupListInerResponse,
}

impl<'a> ISteamUserApi<'a> {
    pub fn new(web_api: &'a mut web_api::WebApi) -> Self {
        return Self { web_api };
    }

    /// Get user friend list
    pub async fn get_friend_list(&'a mut self, steamid: web_api::SteamId) -> Result<Vec<Friend>> {
        let data = self
            .web_api
            .request(&format!(
                "ISteamUser/GetFriendList/v0001/?steamid={}",
                steamid.to_string()
            ))
            .await?;
        let f: FriendListResponse = serde_json::from_slice(&data)?;
        Ok(f.friendslist.friends)
    }

    /// Obtain user profile SteamId by profile id.
    pub async fn resolve_vanity_url(&'a mut self, url: &str) -> Result<web_api::SteamId> {
        let data = self
            .web_api
            .request(&format!(
                "ISteamUser/ResolveVanityURL/v0001/?vanityurl={}",
                url
            ))
            .await?;
        let f: ResolvedUrlResponse = serde_json::from_slice(&data)?;
        if let Some(id) = f.response.steamid {
            let id = id.parse::<web_api::SteamId>()?;
            return Ok(id);
        }
        if let Some(m) = f.response.message {
            bail!("error_code: {}, message: {:?}", f.response.success, m)
        }
        bail!("error_code: {}", f.response.success)
    }

    /// Player ban/probation status
    pub async fn get_player_bans(
        &'a mut self,
        steamids: &[web_api::SteamId],
    ) -> Result<Vec<PlayerBanList>> {
        let data = self
            .web_api
            .request(&format!(
                "ISteamUser/GetPlayerBans/v1/?steamids={}",
                steamids.iter().join(",")
            ))
            .await?;
        let f: PlayerBanListResponse = serde_json::from_slice(&data)?;
        return Ok(f.players);
    }

    /// User profile data
    pub async fn get_player_summaries(
        &'a mut self,
        steamids: &[web_api::SteamId],
    ) -> Result<Vec<PlayerSummaries>> {
        let data = self
            .web_api
            .request(&format!(
                "ISteamUser/GetPlayerSummaries/v0002/?steamids={}",
                steamids.iter().join(",")
            ))
            .await?;
        let f: PlayerSummariesResponse = serde_json::from_slice(&data)?;
        return Ok(f.response.players);
    }

    /// User group list
    pub async fn get_user_group_list(
        &'a mut self,
        steamid: web_api::SteamId,
    ) -> Result<Vec<UserGroup>> {
        let data = self
            .web_api
            .request(&format!(
                "ISteamUser/GetUserGroupList/v1/?steamid={}",
                steamid
            ))
            .await?;
        let f: UserGroupListResponse = serde_json::from_slice(&data)?;
        if !f.response.success {
            bail!("unexpected success value: {}", f.response.success)
        }
        return Ok(f.response.groups);
    }
}
