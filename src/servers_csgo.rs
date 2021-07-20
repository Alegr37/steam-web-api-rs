use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_game_maps_playtime;
pub mod get_game_servers_status;

pub struct ServersCsgo<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> ServersCsgo<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_game_servers_status(
        &'a mut self,
    ) -> Result<get_game_servers_status::Response> {
        let query = format!("?key={}", self.api.get_key()?);
        let data = self
            .api
            .request(&format!(
                "ICSGOServers_730/GetGameServersStatus/v1/{}",
                &query
            ))
            .await?;
        return get_game_servers_status::Response::from(&data);
    }

    pub async fn get_game_maps_playtime(
        &'a mut self,
        interval: &str,
        gamemode: &str,
        mapgroup: &str,
    ) -> Result<get_game_maps_playtime::Response> {
        let query = format!(
            "?key={}&interval={}&gamemode={}&mapgroup={}",
            self.api.get_key()?,
            interval,
            gamemode,
            mapgroup
        );
        let data = self
            .api
            .request(&format!(
                "ICSGOServers_730/GetGameMapsPlaytime/v1/{}",
                &query
            ))
            .await?;
        return get_game_maps_playtime::Response::from(&data);
    }
}
