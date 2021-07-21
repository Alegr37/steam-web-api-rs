use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod create_account;
pub mod delete_account;
pub mod get_account_list;
pub mod get_account_public_info;
pub mod get_server_i_ps_by_steam_id;
pub mod get_server_steam_i_ds_by_ip;
pub mod query_login_token;
pub mod reset_login_token;
pub mod set_memo;

pub async fn set_memo(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    memo: &str,
) -> Result<set_memo::Response> {
    let query = format!("?key={}&steamid={}&memo={}", api.get_key()?, steamid, memo);
    let data = api
        .request(&format!("IGameServersService/SetMemo/v1/{}", &query))
        .await?;
    return set_memo::Response::from(&data);
}

pub async fn reset_login_token(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
) -> Result<reset_login_token::Response> {
    let query = format!("?key={}&steamid={}", api.get_key()?, steamid);
    let data = api
        .request(&format!(
            "IGameServersService/ResetLoginToken/v1/{}",
            &query
        ))
        .await?;
    return reset_login_token::Response::from(&data);
}

pub async fn query_login_token(
    api: &mut crate::WebApi,
    login_token: &str,
) -> Result<query_login_token::Response> {
    let query = format!("?key={}&login_token={}", api.get_key()?, login_token);
    let data = api
        .request(&format!(
            "IGameServersService/QueryLoginToken/v1/{}",
            &query
        ))
        .await?;
    return query_login_token::Response::from(&data);
}

pub async fn get_server_steam_i_ds_by_ip(
    api: &mut crate::WebApi,
    server_ips: &str,
) -> Result<get_server_steam_i_ds_by_ip::Response> {
    let query = format!("?key={}&server_ips={}", api.get_key()?, server_ips);
    let data = api
        .request(&format!(
            "IGameServersService/GetServerSteamIDsByIP/v1/{}",
            &query
        ))
        .await?;
    return get_server_steam_i_ds_by_ip::Response::from(&data);
}

pub async fn get_server_i_ps_by_steam_id(
    api: &mut crate::WebApi,
    server_steamids: u64,
) -> Result<get_server_i_ps_by_steam_id::Response> {
    let query = format!(
        "?key={}&server_steamids={}",
        api.get_key()?,
        server_steamids
    );
    let data = api
        .request(&format!(
            "IGameServersService/GetServerIPsBySteamID/v1/{}",
            &query
        ))
        .await?;
    return get_server_i_ps_by_steam_id::Response::from(&data);
}

pub async fn get_account_public_info(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
) -> Result<get_account_public_info::Response> {
    let query = format!("?key={}&steamid={}", api.get_key()?, steamid);
    let data = api
        .request(&format!(
            "IGameServersService/GetAccountPublicInfo/v1/{}",
            &query
        ))
        .await?;
    return get_account_public_info::Response::from(&data);
}

pub async fn get_account_list(api: &mut crate::WebApi) -> Result<get_account_list::Response> {
    let query = format!("?key={}", api.get_key()?);
    let data = api
        .request(&format!("IGameServersService/GetAccountList/v1/{}", &query))
        .await?;
    return get_account_list::Response::from(&data);
}

pub async fn delete_account(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
) -> Result<delete_account::Response> {
    let query = format!("?key={}&steamid={}", api.get_key()?, steamid);
    let data = api
        .request(&format!("IGameServersService/DeleteAccount/v1/{}", &query))
        .await?;
    return delete_account::Response::from(&data);
}

pub async fn create_account(
    api: &mut crate::WebApi,
    appid: u32,
    memo: &str,
) -> Result<create_account::Response> {
    let query = format!("?key={}&appid={}&memo={}", api.get_key()?, appid, memo);
    let data = api
        .request(&format!("IGameServersService/CreateAccount/v1/{}", &query))
        .await?;
    return create_account::Response::from(&data);
}
