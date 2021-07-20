use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_player_items;
pub mod get_schema;
pub mod get_schema_items;
pub mod get_schema_overview;
pub mod get_schema_url;
pub mod get_store_meta_data;
pub mod get_store_status;

pub struct EconItemsTeamFortress2<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> EconItemsTeamFortress2<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_store_status(&'a mut self) -> Result<get_store_status::Response> {
        let query = format!("?key={}", self.api.get_key()?);
        let data = self
            .api
            .request(&format!("IEconItems_440/GetStoreStatus/v1/{}", &query))
            .await?;
        return get_store_status::Response::from(&data);
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
            .request(&format!("IEconItems_440/GetStoreMetaData/v1/{}", &query))
            .await?;
        return get_store_meta_data::Response::from(&data);
    }

    pub async fn get_schema_url(&'a mut self) -> Result<get_schema_url::Response> {
        let query = format!("?key={}", self.api.get_key()?);
        let data = self
            .api
            .request(&format!("IEconItems_440/GetSchemaURL/v1/{}", &query))
            .await?;
        return get_schema_url::Response::from(&data);
    }

    pub async fn get_schema_overview(
        &'a mut self,
        language: Option<&str>,
    ) -> Result<get_schema_overview::Response> {
        let query = format!(
            "?key={}&language={}",
            self.api.get_key()?,
            language.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("IEconItems_440/GetSchemaOverview/v1/{}", &query))
            .await?;
        return get_schema_overview::Response::from(&data);
    }

    pub async fn get_schema_items(
        &'a mut self,
        language: Option<&str>,
        start: Option<i32>,
    ) -> Result<get_schema_items::Response> {
        let query = format!(
            "?key={}&language={}&start={}",
            self.api.get_key()?,
            language.unwrap_or_default(),
            start.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("IEconItems_440/GetSchemaItems/v1/{}", &query))
            .await?;
        return get_schema_items::Response::from(&data);
    }

    pub async fn get_schema(&'a mut self, language: Option<&str>) -> Result<get_schema::Response> {
        let query = format!(
            "?key={}&language={}",
            self.api.get_key()?,
            language.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("IEconItems_440/GetSchema/v1/{}", &query))
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
            .request(&format!("IEconItems_440/GetPlayerItems/v1/{}", &query))
            .await?;
        return get_player_items::Response::from(&data);
    }
}
