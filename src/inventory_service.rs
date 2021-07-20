use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod combine_item_stacks;
pub mod get_price_sheet;
pub mod split_item_stack;

pub struct InventoryService<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> InventoryService<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn split_item_stack(
        &'a mut self,
        appid: u32,
        itemid: u64,
        quantity: u32,
        steamid: crate::SteamId,
    ) -> Result<split_item_stack::Response> {
        let query = format!(
            "?key={}&appid={}&itemid={}&quantity={}&steamid={}",
            self.api.get_key()?,
            appid,
            itemid,
            quantity,
            steamid
        );
        let data = self
            .api
            .request(&format!("IInventoryService/SplitItemStack/v1/{}", &query))
            .await?;
        return split_item_stack::Response::from(&data);
    }

    pub async fn get_price_sheet(
        &'a mut self,
        ecurrency: i32,
        currency_code: &str,
    ) -> Result<get_price_sheet::Response> {
        let query = format!(
            "?key={}&ecurrency={}&currency_code={}",
            self.api.get_key()?,
            ecurrency,
            currency_code
        );
        let data = self
            .api
            .request(&format!("IInventoryService/GetPriceSheet/v1/{}", &query))
            .await?;
        return get_price_sheet::Response::from(&data);
    }

    pub async fn combine_item_stacks(
        &'a mut self,
        appid: u32,
        fromitemid: u64,
        destitemid: u64,
        quantity: u32,
        steamid: crate::SteamId,
    ) -> Result<combine_item_stacks::Response> {
        let query = format!(
            "?key={}&appid={}&fromitemid={}&destitemid={}&quantity={}&steamid={}",
            self.api.get_key()?,
            appid,
            fromitemid,
            destitemid,
            quantity,
            steamid
        );
        let data = self
            .api
            .request(&format!(
                "IInventoryService/CombineItemStacks/v1/{}",
                &query
            ))
            .await?;
        return combine_item_stacks::Response::from(&data);
    }
}
