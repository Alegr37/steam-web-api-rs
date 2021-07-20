use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_player_items;
pub mod get_schema;

pub struct EconItemsPortal2<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> EconItemsPortal2<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_schema(&'a mut self, language: Option<&str>) -> Result<get_schema::Response> {
        let query = format!(
            "?key={}&language={}",
            self.api.get_key()?,
            language.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("IEconItems_620/GetSchema/v1/{}", &query))
            .await?;
        return get_schema::Response::from(&data);
    }

    pub async fn get_player_items(
        &'a mut self,
        steamid: crate::SteamId,
    ) -> Result<get_player_items::Response> {
        let query = format!("?key={}&steamid={}", self.api.get_key()?, steamid);
        let data = self
            .api
            .request(&format!("IEconItems_620/GetPlayerItems/v1/{}", &query))
            .await?;
        return get_player_items::Response::from(&data);
    }
}
