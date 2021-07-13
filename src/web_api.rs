
use crate::credentials;

mod idota2_automated_tourney_api;
mod idota2_fantasy_api;
mod idota2_match_api;
mod idota2_match_stats_api;
mod idota2_stream_system_api;
mod idota2_teams_api;
mod idota2_ticket_api;
mod i_econ_dota2_api;
mod i_econ_items_api;
mod igc_version_api;
mod i_player_service_api;
mod i_portal2_leaderboards_api;
mod i_steam_apps_api;
mod i_steam_economy_api;
mod i_steam_game_server_account_api;
mod i_steam_news_api;
mod i_steam_o_auth2_api;
mod i_steam_remote_storage_api;
mod i_steam_user_api;
mod i_steam_user_auth_api;
mod i_steam_user_stats_api;
mod i_steam_web_api_util_api;
mod i_steam_web_user_presence_o_auth_api;
mod itf_promos_api;

pub use idota2_automated_tourney_api::*;
pub use idota2_fantasy_api::*;
pub use idota2_match_api::*;
pub use idota2_match_stats_api::*;
pub use idota2_stream_system_api::*;
pub use idota2_teams_api::*;
pub use idota2_ticket_api::*;
pub use i_econ_dota2_api::*;
pub use i_econ_items_api::*;
pub use igc_version_api::*;
pub use i_player_service_api::*;
pub use i_portal2_leaderboards_api::*;
pub use i_steam_apps_api::*;
pub use i_steam_economy_api::*;
pub use i_steam_game_server_account_api::*;
pub use i_steam_news_api::*;
pub use i_steam_o_auth2_api::*;
pub use i_steam_remote_storage_api::*;
pub use i_steam_user_api::*;
pub use i_steam_user_auth_api::*;
pub use i_steam_user_stats_api::*;
pub use i_steam_web_api_util_api::*;
pub use i_steam_web_user_presence_o_auth_api::*;
pub use itf_promos_api::*;

pub enum AppId{}
pub type SteamId = u64;

pub struct WebApiConfig {
    credentials: credentials::CredentialsConfig,
    retry_count: usize,
    web_api_endpoint: String,
}


pub struct WebApi {
    config: WebApiConfig,
}

impl WebApi {
    fn new(config: WebApiConfig) -> WebApi {
        return WebApi{
            config
        }
    }

    fn IDOTA2AutomatedTourney(&mut self , app: AppId) -> IDOTA2AutomatedTourneyApi {
        return IDOTA2AutomatedTourneyApi::new(self, app)
    }
    fn IDOTA2Fantasy(&mut self, app: AppId) -> IDOTA2FantasyApi {
        return IDOTA2FantasyApi::new(self, app)
    }
    fn IDOTA2Match(&mut self, app: AppId) -> IDOTA2MatchApi {
        return IDOTA2MatchApi::new(self, app)
    }
    fn IDOTA2MatchStats(&mut self, app: AppId) -> IDOTA2MatchStatsApi {
        return IDOTA2MatchStatsApi::new(self, app)
    }
    fn IDOTA2StreamSystem(&mut self, app: AppId) -> IDOTA2StreamSystemApi {
        return IDOTA2StreamSystemApi::new(self, app)
    }
    fn IDOTA2Teams(&mut self, app: AppId) -> IDOTA2TeamsApi {
        return IDOTA2TeamsApi::new(self, app)
    }
    fn IDOTA2Ticket(&mut self, app: AppId) -> IDOTA2TicketApi {
        return IDOTA2TicketApi::new(self, app)
    }
    fn IEconDOTA2(&mut self, app: AppId) -> IEconDOTA2Api {
        return IEconDOTA2Api::new(self, app)
    }
    fn IEconItems(&mut self, app: AppId) -> IEconItemsApi {
        return IEconItemsApi::new(self, app)
    }
    fn IGCVersion(&mut self, app: AppId) -> IGCVersionApi {
        return IGCVersionApi::new(self, app)
    }
    fn IPlayerService(&mut self) -> IPlayerServiceApi {
        return IPlayerServiceApi::new(self)
    }
    fn IPortal2Leaderboards(&mut self, app: AppId) -> IPortal2LeaderboardsApi {
        return IPortal2LeaderboardsApi::new(self)
    }
    fn ISteamApps(&mut self) -> ISteamAppsApi {
        return ISteamAppsApi::new(self)
    }
    fn ISteamEconomy(&mut self) -> ISteamEconomyApi {
        return ISteamEconomyApi::new(self)
    }
    fn ISteamGameServerAccount(&mut self) -> ISteamGameServerAccountApi {
        return ISteamGameServerAccountApi::new(self)
    }
    fn ISteamNews(&mut self) -> ISteamNewsApi {
        return ISteamNewsApi::new(self)
    }
    fn ISteamOAuth2(&mut self) -> ISteamOAuth2Api {
        return ISteamOAuth2Api::new(self)
    }
    fn ISteamRemoteStorage(&mut self) -> ISteamRemoteStorageApi {
        return ISteamRemoteStorageApi::new(self)
    }
    fn ISteamUser(&mut self) -> ISteamUserApi {
        return ISteamUserApi::new(self)
    }
    fn ISteamUserAuth(&mut self) -> ISteamUserAuthApi {
        return ISteamUserAuthApi::new(self)
    }
    fn ISteamUserStats(&mut self) -> ISteamUserStatsApi {
        return ISteamUserStatsApi::new(self)
    }
    fn ISteamWebAPIUtil(&mut self) -> ISteamWebAPIUtilApi {
        return ISteamWebAPIUtilApi::new(self)
    }
    fn ISteamWebUserPresenceOAuth(&mut self) -> ISteamWebUserPresenceOAuthApi {
        return ISteamWebUserPresenceOAuthApi::new(self)
    }
    fn ITFPromos(&mut self, app: AppId) -> ITFPromosApi {
        return ITFPromosApi::new(self)
    }
}