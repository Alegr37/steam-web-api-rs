use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_player_items;

pub async fn get_player_items(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
) -> Result<get_player_items::Response> {
    let query = format!("?key={}&steamid={}", api.get_key()?, steamid);
    let data = api
        .request(&format!("IEconItems_221540/GetPlayerItems/v1/{}", &query))
        .await?;
    return get_player_items::Response::from(&data);
}
