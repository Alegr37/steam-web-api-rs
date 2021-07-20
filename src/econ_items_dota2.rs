use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_equipped_player_items;
pub mod get_player_items;
pub mod get_schema_url;
pub mod get_store_meta_data;

pub struct EconItemsDota2<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> EconItemsDota2<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_store_meta_data(
        &'a mut self,
        language: Option<&str>,
    ) -> Result<get_store_meta_data::Response> {
        let query = format!(
            "?key={}&language={}",
            self.api.get_key()?,
            language.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("IEconItems_570/GetStoreMetaData/v1/{}", &query))
            .await?;
        return get_store_meta_data::Response::from(&data);
    }

    pub async fn get_schema_url(&'a mut self) -> Result<get_schema_url::Response> {
        let query = format!("?key={}", self.api.get_key()?);
        let data = self
            .api
            .request(&format!("IEconItems_570/GetSchemaURL/v1/{}", &query))
            .await?;
        return get_schema_url::Response::from(&data);
    }

    pub async fn get_player_items(
        &'a mut self,
        steamid: crate::SteamId,
    ) -> Result<get_player_items::Response> {
        let query = format!("?key={}&steamid={}", self.api.get_key()?, steamid);
        let data = self
            .api
            .request(&format!("IEconItems_570/GetPlayerItems/v1/{}", &query))
            .await?;
        return get_player_items::Response::from(&data);
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
                "IEconItems_570/GetEquippedPlayerItems/v1/{}",
                &query
            ))
            .await?;
        return get_equipped_player_items::Response::from(&data);
    }
}
