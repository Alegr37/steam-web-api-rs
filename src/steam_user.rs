use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_friend_list;
pub mod get_player_bans;
pub mod get_player_summaries;
pub mod get_user_group_list;
pub mod resolve_vanity_url;

pub struct SteamUser<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> SteamUser<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn resolve_vanity_url(
        &'a mut self,
        vanityurl: &str,
        url_type: Option<i32>,
    ) -> Result<resolve_vanity_url::Response> {
        let query = format!(
            "?key={}&vanityurl={}&url_type={}",
            self.api.get_key()?,
            vanityurl,
            url_type.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("ISteamUser/ResolveVanityURL/v1/{}", &query))
            .await?;
        return resolve_vanity_url::Response::from(&data);
    }

    pub async fn get_user_group_list(
        &'a mut self,
        steamid: crate::SteamId,
    ) -> Result<get_user_group_list::Response> {
        let query = format!("?key={}&steamid={}", self.api.get_key()?, steamid);
        let data = self
            .api
            .request(&format!("ISteamUser/GetUserGroupList/v1/{}", &query))
            .await?;
        return get_user_group_list::Response::from(&data);
    }

    pub async fn get_player_summaries(
        &'a mut self,
        steamids: Vec<crate::SteamId>,
    ) -> Result<get_player_summaries::Response> {
        let query = format!(
            "?key={}&steamids={}",
            self.api.get_key()?,
            steamids.iter().join(",")
        );
        let data = self
            .api
            .request(&format!("ISteamUser/GetPlayerSummaries/v2/{}", &query))
            .await?;
        return get_player_summaries::Response::from(&data);
    }

    pub async fn get_player_bans(
        &'a mut self,
        steamids: Vec<crate::SteamId>,
    ) -> Result<get_player_bans::Response> {
        let query = format!(
            "?key={}&steamids={}",
            self.api.get_key()?,
            steamids.iter().join(",")
        );
        let data = self
            .api
            .request(&format!("ISteamUser/GetPlayerBans/v1/{}", &query))
            .await?;
        return get_player_bans::Response::from(&data);
    }

    pub async fn get_friend_list(
        &'a mut self,
        steamid: crate::SteamId,
        relationship: Option<&str>,
    ) -> Result<get_friend_list::Response> {
        let query = format!(
            "?key={}&steamid={}&relationship={}",
            self.api.get_key()?,
            steamid,
            relationship.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("ISteamUser/GetFriendList/v1/{}", &query))
            .await?;
        return get_friend_list::Response::from(&data);
    }
}
