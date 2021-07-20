use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod poll_status;

pub struct SteamWebUserPresenceOAuth<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> SteamWebUserPresenceOAuth<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn poll_status(
        &'a mut self,
        steamid: crate::SteamId,
        umqid: u64,
        message: u32,
        pollid: Option<u32>,
        sectimeout: Option<u32>,
        secidletime: Option<u32>,
        use_accountids: Option<u32>,
    ) -> Result<poll_status::Response> {
        let query = format!("?key={}&steamid={}&umqid={}&message={}&pollid={}&sectimeout={}&secidletime={}&use_accountids={}", self.api.get_key()?, steamid, umqid, message, pollid.unwrap_or_default(), sectimeout.unwrap_or_default(), secidletime.unwrap_or_default(), use_accountids.unwrap_or_default());
        let data = self
            .api
            .request(&format!(
                "ISteamWebUserPresenceOAuth/PollStatus/v1/{}",
                &query
            ))
            .await?;
        return poll_status::Response::from(&data);
    }
}
