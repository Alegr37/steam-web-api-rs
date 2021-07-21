use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod authenticate_user;
pub mod authenticate_user_ticket;

pub async fn authenticate_user_ticket(
    api: &mut crate::WebApi,
    appid: u32,
    ticket: &str,
) -> Result<authenticate_user_ticket::Response> {
    let query = format!("?key={}&appid={}&ticket={}", api.get_key()?, appid, ticket);
    let data = api
        .request(&format!(
            "ISteamUserAuth/AuthenticateUserTicket/v1/{}",
            &query
        ))
        .await?;
    return authenticate_user_ticket::Response::from(&data);
}

pub async fn authenticate_user(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    sessionkey: String,
    encrypted_loginkey: String,
) -> Result<authenticate_user::Response> {
    let query = format!(
        "?key={}&steamid={}&sessionkey={}&encrypted_loginkey={}",
        api.get_key()?,
        steamid,
        sessionkey,
        encrypted_loginkey
    );
    let data = api
        .request(&format!("ISteamUserAuth/AuthenticateUser/v1/{}", &query))
        .await?;
    return authenticate_user::Response::from(&data);
}
