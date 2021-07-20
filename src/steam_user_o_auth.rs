use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_token_details;

pub struct SteamUserOAuth<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> SteamUserOAuth<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_token_details(
        &'a mut self,
        access_token: &str,
    ) -> Result<get_token_details::Response> {
        let query = format!("?key={}&access_token={}", self.api.get_key()?, access_token);
        let data = self
            .api
            .request(&format!("ISteamUserOAuth/GetTokenDetails/v1/{}", &query))
            .await?;
        return get_token_details::Response::from(&data);
    }
}
