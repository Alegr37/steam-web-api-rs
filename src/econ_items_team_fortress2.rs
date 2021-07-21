use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_player_items;
pub mod get_schema;
pub mod get_schema_items;
pub mod get_schema_overview;
pub mod get_schema_url;
pub mod get_store_meta_data;
pub mod get_store_status;

pub async fn get_store_status(api: &mut crate::WebApi) -> Result<get_store_status::Response> {
    let query = format!("?key={}", api.get_key()?);
    let data = api
        .request(&format!("IEconItems_440/GetStoreStatus/v1/{}", &query))
        .await?;
    return get_store_status::Response::from(&data);
}

pub async fn get_store_meta_data(
    api: &mut crate::WebApi,
    language: Option<&str>,
) -> Result<get_store_meta_data::Response> {
    let query = format!(
        "?key={}&language={}",
        api.get_key()?,
        language.unwrap_or_default()
    );
    let data = api
        .request(&format!("IEconItems_440/GetStoreMetaData/v1/{}", &query))
        .await?;
    return get_store_meta_data::Response::from(&data);
}

pub async fn get_schema_url(api: &mut crate::WebApi) -> Result<get_schema_url::Response> {
    let query = format!("?key={}", api.get_key()?);
    let data = api
        .request(&format!("IEconItems_440/GetSchemaURL/v1/{}", &query))
        .await?;
    return get_schema_url::Response::from(&data);
}

pub async fn get_schema_overview(
    api: &mut crate::WebApi,
    language: Option<&str>,
) -> Result<get_schema_overview::Response> {
    let query = format!(
        "?key={}&language={}",
        api.get_key()?,
        language.unwrap_or_default()
    );
    let data = api
        .request(&format!("IEconItems_440/GetSchemaOverview/v1/{}", &query))
        .await?;
    return get_schema_overview::Response::from(&data);
}

pub async fn get_schema_items(
    api: &mut crate::WebApi,
    language: Option<&str>,
    start: Option<i32>,
) -> Result<get_schema_items::Response> {
    let query = format!(
        "?key={}&language={}&start={}",
        api.get_key()?,
        language.unwrap_or_default(),
        start.unwrap_or_default()
    );
    let data = api
        .request(&format!("IEconItems_440/GetSchemaItems/v1/{}", &query))
        .await?;
    return get_schema_items::Response::from(&data);
}

pub async fn get_schema(
    api: &mut crate::WebApi,
    language: Option<&str>,
) -> Result<get_schema::Response> {
    let query = format!(
        "?key={}&language={}",
        api.get_key()?,
        language.unwrap_or_default()
    );
    let data = api
        .request(&format!("IEconItems_440/GetSchema/v1/{}", &query))
        .await?;
    return get_schema::Response::from(&data);
}

pub async fn get_player_items(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
) -> Result<get_player_items::Response> {
    let query = format!("?key={}&steamid={}", api.get_key()?, steamid);
    let data = api
        .request(&format!("IEconItems_440/GetPlayerItems/v1/{}", &query))
        .await?;
    return get_player_items::Response::from(&data);
}
