use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_player_items;

pub struct EconItemsBattleBlockTheater<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> EconItemsBattleBlockTheater<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_player_items(
        &'a mut self,
        steamid: crate::SteamId,
    ) -> Result<get_player_items::Response> {
        let query = format!("?key={}&steamid={}", self.api.get_key()?, steamid);
        let data = self
            .api
            .request(&format!("IEconItems_238460/GetPlayerItems/v1/{}", &query))
            .await?;
        return get_player_items::Response::from(&data);
    }
}
