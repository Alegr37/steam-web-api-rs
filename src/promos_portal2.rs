use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_item_id;
pub mod grant_item;

pub struct PromosPortal2<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> PromosPortal2<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn grant_item(
        &'a mut self,
        steamid: crate::SteamId,
        promo_id: u32,
    ) -> Result<grant_item::Response> {
        let query = format!(
            "?key={}&steamid={}&PromoID={}",
            self.api.get_key()?,
            steamid,
            promo_id
        );
        let data = self
            .api
            .request(&format!("ITFPromos_620/GrantItem/v1/{}", &query))
            .await?;
        return grant_item::Response::from(&data);
    }

    pub async fn get_item_id(
        &'a mut self,
        steamid: crate::SteamId,
        promo_id: u32,
    ) -> Result<get_item_id::Response> {
        let query = format!(
            "?key={}&steamid={}&PromoID={}",
            self.api.get_key()?,
            steamid,
            promo_id
        );
        let data = self
            .api
            .request(&format!("ITFPromos_620/GetItemID/v1/{}", &query))
            .await?;
        return get_item_id::Response::from(&data);
    }
}
