use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_token_details;

pub async fn get_token_details(
    api: &mut crate::WebApi,
    access_token: &str,
) -> Result<get_token_details::Response> {
    let query = format!("?key={}&access_token={}", api.get_key()?, access_token);
    let data = api
        .request(&format!("ISteamUserOAuth/GetTokenDetails/v1/{}", &query))
        .await?;
    return get_token_details::Response::from(&data);
}
