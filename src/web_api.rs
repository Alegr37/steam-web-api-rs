pub use crate::credentials::*;
use anyhow::{bail, Result};
use bytes::Bytes;
use thiserror::Error;
use url::Url;

mod dota2_automated_tourney_api;
mod dota2_fantasy_api;
mod dota2_match_api;
mod dota2_match_stats_api;
mod dota2_stream_system_api;
mod dota2_teams_api;
mod dota2_ticket_api;
mod econ_dota2_api;
mod econ_items_api;
mod gc_version_api;
mod player_service_api;
mod portal2_leaderboards_api;
mod steam_apps_api;
mod steam_economy_api;
mod steam_game_server_account_api;
mod steam_news_api;
mod steam_o_auth2_api;
mod steam_remote_storage_api;
mod steam_user_api;
mod steam_user_auth_api;
mod steam_user_stats_api;
mod steam_web_api_util_api;
mod steam_web_user_presence_o_auth_api;
mod tf_promos_api;

pub use dota2_automated_tourney_api::*;
pub use dota2_fantasy_api::*;
pub use dota2_match_api::*;
pub use dota2_match_stats_api::*;
pub use dota2_stream_system_api::*;
pub use dota2_teams_api::*;
pub use dota2_ticket_api::*;
pub use econ_dota2_api::*;
pub use econ_items_api::*;
pub use gc_version_api::*;
pub use player_service_api::*;
pub use portal2_leaderboards_api::*;
pub use steam_apps_api::*;
pub use steam_economy_api::*;
pub use steam_game_server_account_api::*;
pub use steam_news_api::*;
pub use steam_o_auth2_api::*;
pub use steam_remote_storage_api::*;
pub use steam_user_api::*;
pub use steam_user_auth_api::*;
pub use steam_user_stats_api::*;
pub use steam_web_api_util_api::*;
pub use steam_web_user_presence_o_auth_api::*;
pub use tf_promos_api::*;

pub enum AppId {}
pub type SteamId = u64;
pub type GroupId = u64;

pub struct WebApiConfig {
    pub retry_count: usize,
    pub endpoint: Url,
}

pub struct WebApi {
    credentials: Box<dyn ApiKeyProvider>,
    config: WebApiConfig,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("connection timeout")]
    ConnectError,
    #[error("read timeout")]
    ReadError,
    #[error("internal server error")]
    InternalServerError,
    #[error("auth error")]
    AuthError,
    #[error("unexpected url")]
    UrlError,
    #[error("can't get credentials")]
    CredentialsError,
}

impl WebApi {
    pub fn new(config: WebApiConfig, credentials: Box<dyn ApiKeyProvider>) -> WebApi {
        return WebApi {
            credentials,
            config,
        };
    }

    fn build_url(&mut self, path: &str) -> Result<Url, Error> {
        let mut url = match self.config.endpoint.join(path) {
            Ok(url) => url,
            _ => return Err(Error::UrlError),
        };
        let key = match self.credentials.get_key() {
            Ok(key) => key,
            _ => return Err(Error::CredentialsError),
        };
        url.query_pairs_mut().extend_pairs(&[("key", key)]);
        Ok(url)
    }

    async fn request_impl(&mut self, url: &Url) -> Result<Bytes, Error> {
        let response = match reqwest::get(url.as_str()).await {
            Ok(response) => response,
            _ => return Err(Error::ConnectError),
        };
        let status = response.status();
        if !status.is_success() {
            if status == reqwest::StatusCode::FORBIDDEN {
                return Err(Error::AuthError);
            }
            return Err(Error::InternalServerError);
        }
        match response.bytes().await {
            Ok(data) => Ok(data),
            _ => return Err(Error::ReadError),
        }
    }

    async fn request(&mut self, path: &str) -> Result<Bytes, Error> {
        let url = self.build_url(path)?;
        let mut retry_count = 0;
        while retry_count < self.config.retry_count {
            let res = self.request_impl(&url).await;
            if let Ok(data) = res {
                return Ok(data);
            }
            retry_count += 1
        }
        return self.request_impl(&url).await;
    }

    pub fn dota2_automated_tourney(&mut self, app: AppId) -> IDOTA2AutomatedTourneyApi {
        return IDOTA2AutomatedTourneyApi::new(self, app);
    }
    pub fn dota2_fantasy(&mut self, app: AppId) -> IDOTA2FantasyApi {
        return IDOTA2FantasyApi::new(self, app);
    }
    pub fn dota2_match(&mut self, app: AppId) -> IDOTA2MatchApi {
        return IDOTA2MatchApi::new(self, app);
    }
    pub fn dota2_match_stats(&mut self, app: AppId) -> IDOTA2MatchStatsApi {
        return IDOTA2MatchStatsApi::new(self, app);
    }
    pub fn dota2_stream_system(&mut self, app: AppId) -> IDOTA2StreamSystemApi {
        return IDOTA2StreamSystemApi::new(self, app);
    }
    pub fn dota2_teams(&mut self, app: AppId) -> IDOTA2TeamsApi {
        return IDOTA2TeamsApi::new(self, app);
    }
    pub fn dota2_ticket(&mut self, app: AppId) -> IDOTA2TicketApi {
        return IDOTA2TicketApi::new(self, app);
    }
    pub fn econ_dota2(&mut self, app: AppId) -> IEconDOTA2Api {
        return IEconDOTA2Api::new(self, app);
    }
    pub fn econ_items(&mut self, app: AppId) -> IEconItemsApi {
        return IEconItemsApi::new(self, app);
    }
    pub fn gc_version(&mut self, app: AppId) -> IGCVersionApi {
        return IGCVersionApi::new(self, app);
    }
    pub fn player_service(&mut self) -> IPlayerServiceApi {
        return IPlayerServiceApi::new(self);
    }
    pub fn portal2_leaderboards(&mut self) -> IPortal2LeaderboardsApi {
        return IPortal2LeaderboardsApi::new(self);
    }
    pub fn steam_apps(&mut self) -> ISteamAppsApi {
        return ISteamAppsApi::new(self);
    }
    pub fn steam_economy(&mut self) -> ISteamEconomyApi {
        return ISteamEconomyApi::new(self);
    }
    pub fn steam_game_server_account(&mut self) -> ISteamGameServerAccountApi {
        return ISteamGameServerAccountApi::new(self);
    }
    pub fn steam_news(&mut self) -> ISteamNewsApi {
        return ISteamNewsApi::new(self);
    }
    pub fn steam_o_auth2(&mut self) -> ISteamOAuth2Api {
        return ISteamOAuth2Api::new(self);
    }
    pub fn steam_remote_storage(&mut self) -> ISteamRemoteStorageApi {
        return ISteamRemoteStorageApi::new(self);
    }
    pub fn steam_user(&mut self) -> ISteamUserApi {
        return ISteamUserApi::new(self);
    }
    pub fn steam_user_auth(&mut self) -> ISteamUserAuthApi {
        return ISteamUserAuthApi::new(self);
    }
    pub fn steam_user_stats(&mut self) -> ISteamUserStatsApi {
        return ISteamUserStatsApi::new(self);
    }
    pub fn steam_web_api_util(&mut self) -> ISteamWebAPIUtilApi {
        return ISteamWebAPIUtilApi::new(self);
    }
    pub fn steam_web_user_presence_o_auth(&mut self) -> ISteamWebUserPresenceOAuthApi {
        return ISteamWebUserPresenceOAuthApi::new(self);
    }
    pub fn tf_promos(&mut self) -> ITFPromosApi {
        return ITFPromosApi::new(self);
    }
}
