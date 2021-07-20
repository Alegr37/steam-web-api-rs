use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_equipped_player_items;

pub struct EconItemsArtifact<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> EconItemsArtifact<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_equipped_player_items(
        &'a mut self,
        steamid: crate::SteamId,
        class_id: u32,
    ) -> Result<get_equipped_player_items::Response> {
        let query = format!(
            "?key={}&steamid={}&class_id={}",
            self.api.get_key()?,
            steamid,
            class_id
        );
        let data = self
            .api
            .request(&format!(
                "IEconItems_583950/GetEquippedPlayerItems/v1/{}",
                &query
            ))
            .await?;
        return get_equipped_player_items::Response::from(&data);
    }
}
