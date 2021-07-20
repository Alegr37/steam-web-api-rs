pub mod common;
pub mod credentials;

use anyhow::Result;
use bytes::Bytes;
pub use common::*;
pub use credentials::*;
use thiserror::Error;
use url::Url;

pub mod client_stats_dota_underlords;
pub use client_stats_dota_underlords::*;

pub mod players_csgo;
pub use players_csgo::*;

pub mod servers_csgo;
pub use servers_csgo::*;

pub mod tournaments_csgo;
pub use tournaments_csgo::*;

pub mod fantasy_dota2_beta;
pub use fantasy_dota2_beta::*;

pub mod match_stats_dota2_beta;
pub use match_stats_dota2_beta::*;

pub mod match_stats_dota2;
pub use match_stats_dota2::*;

pub mod match_dota2_beta;
pub use match_dota2_beta::*;

pub mod match_dota2;
pub use match_dota2::*;

pub mod stream_system_dota2_beta;
pub use stream_system_dota2_beta::*;

pub mod stream_system_dota2;
pub use stream_system_dota2::*;

pub mod ticket_dota2_beta;
pub use ticket_dota2_beta::*;

pub mod ticket_dota2;
pub use ticket_dota2::*;

pub mod econ_dota2_beta;
pub use econ_dota2_beta::*;

pub mod econ_dota2;
pub use econ_dota2::*;

pub mod econ_items_dota_underlords;
pub use econ_items_dota_underlords::*;

pub mod econ_items_artifact_foundry;
pub use econ_items_artifact_foundry::*;

pub mod econ_items_dota2_beta;
pub use econ_items_dota2_beta::*;

pub mod econ_items_defense_grid2;
pub use econ_items_defense_grid2::*;

pub mod econ_items_battle_block_theater;
pub use econ_items_battle_block_theater::*;

pub mod econ_items_team_fortress2;
pub use econ_items_team_fortress2::*;

pub mod econ_items_dota2;
pub use econ_items_dota2::*;

pub mod econ_items_artifact;
pub use econ_items_artifact::*;

pub mod econ_items_portal2;
pub use econ_items_portal2::*;

pub mod econ_items_csgo;
pub use econ_items_csgo::*;

pub mod gc_version_dota_underlords;
pub use gc_version_dota_underlords::*;

pub mod gc_version_artifact_foundry;
pub use gc_version_artifact_foundry::*;

pub mod gc_version_dota2_beta;
pub use gc_version_dota2_beta::*;

pub mod gc_version_team_fortress2;
pub use gc_version_team_fortress2::*;

pub mod gc_version_dota2;
pub use gc_version_dota2::*;

pub mod gc_version_artifact;
pub use gc_version_artifact::*;

pub mod gc_version_csgo;
pub use gc_version_csgo::*;

pub mod portal2_leaderboards_portal2;
pub use portal2_leaderboards_portal2::*;

pub mod steam_apps;
pub use steam_apps::*;

pub mod steam_broadcast;
pub use steam_broadcast::*;

pub mod steam_cdn;
pub use steam_cdn::*;

pub mod steam_directory;
pub use steam_directory::*;

pub mod steam_economy;
pub use steam_economy::*;

pub mod steam_news;
pub use steam_news::*;

pub mod steam_remote_storage;
pub use steam_remote_storage::*;

pub mod steam_user;
pub use steam_user::*;

pub mod steam_user_auth;
pub use steam_user_auth::*;

pub mod steam_user_o_auth;
pub use steam_user_o_auth::*;

pub mod steam_user_stats;
pub use steam_user_stats::*;

pub mod steam_web_api_util;
pub use steam_web_api_util::*;

pub mod steam_web_user_presence_o_auth;
pub use steam_web_user_presence_o_auth::*;

pub mod items_team_fortress2;
pub use items_team_fortress2::*;

pub mod promos_dota2_beta;
pub use promos_dota2_beta::*;

pub mod promos_team_fortress2;
pub use promos_team_fortress2::*;

pub mod promos_dota2;
pub use promos_dota2::*;

pub mod promos_portal2;
pub use promos_portal2::*;

pub mod system_team_fortress2;
pub use system_team_fortress2::*;

pub mod game_servers_service;
pub use game_servers_service::*;

pub mod broadcast_service;
pub use broadcast_service::*;

pub mod content_server_config_service;
pub use content_server_config_service::*;

pub mod content_server_directory_service;
pub use content_server_directory_service::*;

pub mod published_file_service;
pub use published_file_service::*;

pub mod econ_service;
pub use econ_service::*;

pub mod player_service;
pub use player_service::*;

pub mod game_notifications_service;
pub use game_notifications_service::*;

pub mod inventory_service;
pub use inventory_service::*;

pub mod store_service;
pub use store_service::*;

pub mod cheat_reporting_service;
pub use cheat_reporting_service::*;

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

pub struct WebApiConfig {
    pub retry_count: usize,
    pub endpoint: Url,
}

pub struct WebApi {
    credentials: Box<dyn ApiKeyProvider>,
    config: WebApiConfig,
}

impl WebApi {
    pub fn new(config: WebApiConfig, credentials: Box<dyn ApiKeyProvider>) -> WebApi {
        return WebApi {
            credentials,
            config,
        };
    }

    fn get_key(&self) -> Result<&WebApiKey> {
        return self.credentials.get_key();
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
        let url = match self.config.endpoint.join(path) {
            Ok(url) => url,
            _ => return Err(Error::UrlError),
        };
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

    pub fn client_stats_dota_underlords(&mut self) -> ClientStatsDotaUnderlords {
        return ClientStatsDotaUnderlords::new(self);
    }

    pub fn players_csgo(&mut self) -> PlayersCsgo {
        return PlayersCsgo::new(self);
    }

    pub fn servers_csgo(&mut self) -> ServersCsgo {
        return ServersCsgo::new(self);
    }

    pub fn tournaments_csgo(&mut self) -> TournamentsCsgo {
        return TournamentsCsgo::new(self);
    }

    pub fn fantasy_dota2_beta(&mut self) -> FantasyDota2Beta {
        return FantasyDota2Beta::new(self);
    }

    pub fn match_stats_dota2_beta(&mut self) -> MatchStatsDota2Beta {
        return MatchStatsDota2Beta::new(self);
    }

    pub fn match_stats_dota2(&mut self) -> MatchStatsDota2 {
        return MatchStatsDota2::new(self);
    }

    pub fn match_dota2_beta(&mut self) -> MatchDota2Beta {
        return MatchDota2Beta::new(self);
    }

    pub fn match_dota2(&mut self) -> MatchDota2 {
        return MatchDota2::new(self);
    }

    pub fn stream_system_dota2_beta(&mut self) -> StreamSystemDota2Beta {
        return StreamSystemDota2Beta::new(self);
    }

    pub fn stream_system_dota2(&mut self) -> StreamSystemDota2 {
        return StreamSystemDota2::new(self);
    }

    pub fn ticket_dota2_beta(&mut self) -> TicketDota2Beta {
        return TicketDota2Beta::new(self);
    }

    pub fn ticket_dota2(&mut self) -> TicketDota2 {
        return TicketDota2::new(self);
    }

    pub fn econ_dota2_beta(&mut self) -> EconDota2Beta {
        return EconDota2Beta::new(self);
    }

    pub fn econ_dota2(&mut self) -> EconDota2 {
        return EconDota2::new(self);
    }

    pub fn econ_items_dota_underlords(&mut self) -> EconItemsDotaUnderlords {
        return EconItemsDotaUnderlords::new(self);
    }

    pub fn econ_items_artifact_foundry(&mut self) -> EconItemsArtifactFoundry {
        return EconItemsArtifactFoundry::new(self);
    }

    pub fn econ_items_dota2_beta(&mut self) -> EconItemsDota2Beta {
        return EconItemsDota2Beta::new(self);
    }

    pub fn econ_items_defense_grid2(&mut self) -> EconItemsDefenseGrid2 {
        return EconItemsDefenseGrid2::new(self);
    }

    pub fn econ_items_battle_block_theater(&mut self) -> EconItemsBattleBlockTheater {
        return EconItemsBattleBlockTheater::new(self);
    }

    pub fn econ_items_team_fortress2(&mut self) -> EconItemsTeamFortress2 {
        return EconItemsTeamFortress2::new(self);
    }

    pub fn econ_items_dota2(&mut self) -> EconItemsDota2 {
        return EconItemsDota2::new(self);
    }

    pub fn econ_items_artifact(&mut self) -> EconItemsArtifact {
        return EconItemsArtifact::new(self);
    }

    pub fn econ_items_portal2(&mut self) -> EconItemsPortal2 {
        return EconItemsPortal2::new(self);
    }

    pub fn econ_items_csgo(&mut self) -> EconItemsCsgo {
        return EconItemsCsgo::new(self);
    }

    pub fn gc_version_dota_underlords(&mut self) -> GcVersionDotaUnderlords {
        return GcVersionDotaUnderlords::new(self);
    }

    pub fn gc_version_artifact_foundry(&mut self) -> GcVersionArtifactFoundry {
        return GcVersionArtifactFoundry::new(self);
    }

    pub fn gc_version_dota2_beta(&mut self) -> GcVersionDota2Beta {
        return GcVersionDota2Beta::new(self);
    }

    pub fn gc_version_team_fortress2(&mut self) -> GcVersionTeamFortress2 {
        return GcVersionTeamFortress2::new(self);
    }

    pub fn gc_version_dota2(&mut self) -> GcVersionDota2 {
        return GcVersionDota2::new(self);
    }

    pub fn gc_version_artifact(&mut self) -> GcVersionArtifact {
        return GcVersionArtifact::new(self);
    }

    pub fn gc_version_csgo(&mut self) -> GcVersionCsgo {
        return GcVersionCsgo::new(self);
    }

    pub fn portal2_leaderboards_portal2(&mut self) -> Portal2LeaderboardsPortal2 {
        return Portal2LeaderboardsPortal2::new(self);
    }

    pub fn steam_apps(&mut self) -> SteamApps {
        return SteamApps::new(self);
    }

    pub fn steam_broadcast(&mut self) -> SteamBroadcast {
        return SteamBroadcast::new(self);
    }

    pub fn steam_cdn(&mut self) -> SteamCdn {
        return SteamCdn::new(self);
    }

    pub fn steam_directory(&mut self) -> SteamDirectory {
        return SteamDirectory::new(self);
    }

    pub fn steam_economy(&mut self) -> SteamEconomy {
        return SteamEconomy::new(self);
    }

    pub fn steam_news(&mut self) -> SteamNews {
        return SteamNews::new(self);
    }

    pub fn steam_remote_storage(&mut self) -> SteamRemoteStorage {
        return SteamRemoteStorage::new(self);
    }

    pub fn steam_user(&mut self) -> SteamUser {
        return SteamUser::new(self);
    }

    pub fn steam_user_auth(&mut self) -> SteamUserAuth {
        return SteamUserAuth::new(self);
    }

    pub fn steam_user_o_auth(&mut self) -> SteamUserOAuth {
        return SteamUserOAuth::new(self);
    }

    pub fn steam_user_stats(&mut self) -> SteamUserStats {
        return SteamUserStats::new(self);
    }

    pub fn steam_web_api_util(&mut self) -> SteamWebApiUtil {
        return SteamWebApiUtil::new(self);
    }

    pub fn steam_web_user_presence_o_auth(&mut self) -> SteamWebUserPresenceOAuth {
        return SteamWebUserPresenceOAuth::new(self);
    }

    pub fn items_team_fortress2(&mut self) -> ItemsTeamFortress2 {
        return ItemsTeamFortress2::new(self);
    }

    pub fn promos_dota2_beta(&mut self) -> PromosDota2Beta {
        return PromosDota2Beta::new(self);
    }

    pub fn promos_team_fortress2(&mut self) -> PromosTeamFortress2 {
        return PromosTeamFortress2::new(self);
    }

    pub fn promos_dota2(&mut self) -> PromosDota2 {
        return PromosDota2::new(self);
    }

    pub fn promos_portal2(&mut self) -> PromosPortal2 {
        return PromosPortal2::new(self);
    }

    pub fn system_team_fortress2(&mut self) -> SystemTeamFortress2 {
        return SystemTeamFortress2::new(self);
    }

    pub fn game_servers_service(&mut self) -> GameServersService {
        return GameServersService::new(self);
    }

    pub fn broadcast_service(&mut self) -> BroadcastService {
        return BroadcastService::new(self);
    }

    pub fn content_server_config_service(&mut self) -> ContentServerConfigService {
        return ContentServerConfigService::new(self);
    }

    pub fn content_server_directory_service(&mut self) -> ContentServerDirectoryService {
        return ContentServerDirectoryService::new(self);
    }

    pub fn published_file_service(&mut self) -> PublishedFileService {
        return PublishedFileService::new(self);
    }

    pub fn econ_service(&mut self) -> EconService {
        return EconService::new(self);
    }

    pub fn player_service(&mut self) -> PlayerService {
        return PlayerService::new(self);
    }

    pub fn game_notifications_service(&mut self) -> GameNotificationsService {
        return GameNotificationsService::new(self);
    }

    pub fn inventory_service(&mut self) -> InventoryService {
        return InventoryService::new(self);
    }

    pub fn store_service(&mut self) -> StoreService {
        return StoreService::new(self);
    }

    pub fn cheat_reporting_service(&mut self) -> CheatReportingService {
        return CheatReportingService::new(self);
    }
}
