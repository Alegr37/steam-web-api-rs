use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_asset_class_info;
pub mod get_asset_prices;

pub struct SteamEconomy<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> SteamEconomy<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_asset_prices(
        &'a mut self,
        appid: u32,
        currency: Option<&str>,
        language: Option<&str>,
    ) -> Result<get_asset_prices::Response> {
        let query = format!(
            "?key={}&appid={}&currency={}&language={}",
            self.api.get_key()?,
            appid,
            currency.unwrap_or_default(),
            language.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("ISteamEconomy/GetAssetPrices/v1/{}", &query))
            .await?;
        return get_asset_prices::Response::from(&data);
    }

    pub async fn get_asset_class_info(
        &'a mut self,
        appid: u32,
        language: Option<&str>,
        class_count: u32,
        classid0: u64,
        instanceid0: Option<u64>,
    ) -> Result<get_asset_class_info::Response> {
        let query = format!(
            "?key={}&appid={}&language={}&class_count={}&classid0={}&instanceid0={}",
            self.api.get_key()?,
            appid,
            language.unwrap_or_default(),
            class_count,
            classid0,
            instanceid0.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("ISteamEconomy/GetAssetClassInfo/v1/{}", &query))
            .await?;
        return get_asset_class_info::Response::from(&data);
    }
}
