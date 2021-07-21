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
}
