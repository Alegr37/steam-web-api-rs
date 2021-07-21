use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_next_match_sharing_code;

pub async fn get_next_match_sharing_code(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    steamidkey: &str,
    knowncode: &str,
) -> Result<get_next_match_sharing_code::Response> {
    let query = format!(
        "?key={}&steamid={}&steamidkey={}&knowncode={}",
        api.get_key()?,
        steamid,
        steamidkey,
        knowncode
    );
    let data = api
        .request(&format!(
            "ICSGOPlayers_730/GetNextMatchSharingCode/v1/{}",
            &query
        ))
        .await?;
    return get_next_match_sharing_code::Response::from(&data);
}
