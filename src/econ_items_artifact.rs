use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_equipped_player_items;

pub async fn get_equipped_player_items(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    class_id: u32,
) -> Result<get_equipped_player_items::Response> {
    let query = format!(
        "?key={}&steamid={}&class_id={}",
        api.get_key()?,
        steamid,
        class_id
    );
    let data = api
        .request(&format!(
            "IEconItems_583950/GetEquippedPlayerItems/v1/{}",
            &query
        ))
        .await?;
    return get_equipped_player_items::Response::from(&data);
}
