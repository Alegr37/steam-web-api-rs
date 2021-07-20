use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_next_match_sharing_code;

pub struct PlayersCsgo<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> PlayersCsgo<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_next_match_sharing_code(
        &'a mut self,
        steamid: crate::SteamId,
        steamidkey: &str,
        knowncode: &str,
    ) -> Result<get_next_match_sharing_code::Response> {
        let query = format!(
            "?key={}&steamid={}&steamidkey={}&knowncode={}",
            self.api.get_key()?,
            steamid,
            steamidkey,
            knowncode
        );
        let data = self
            .api
            .request(&format!(
                "ICSGOPlayers_730/GetNextMatchSharingCode/v1/{}",
                &query
            ))
            .await?;
        return get_next_match_sharing_code::Response::from(&data);
    }
}
