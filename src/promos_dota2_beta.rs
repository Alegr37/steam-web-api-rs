use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_item_id;
pub mod grant_item;

pub async fn grant_item(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    promoid: u32,
) -> Result<grant_item::Response> {
    let query = format!(
        "?key={}&steamid={}&promoid={}",
        api.get_key()?,
        steamid,
        promoid
    );
    let data = api
        .request(&format!("ITFPromos_205790/GrantItem/v1/{}", &query))
        .await?;
    return grant_item::Response::from(&data);
}

pub async fn get_item_id(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    promoid: u32,
) -> Result<get_item_id::Response> {
    let query = format!(
        "?key={}&steamid={}&promoid={}",
        api.get_key()?,
        steamid,
        promoid
    );
    let data = api
        .request(&format!("ITFPromos_205790/GetItemID/v1/{}", &query))
        .await?;
    return get_item_id::Response::from(&data);
}
