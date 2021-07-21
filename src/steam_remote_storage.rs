use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_collection_details;
pub mod get_published_file_details;
pub mod get_ugc_file_details;

pub async fn get_ugc_file_details(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    ugcid: u64,
    appid: u32,
) -> Result<get_ugc_file_details::Response> {
    let query = format!(
        "?key={}&steamid={}&ugcid={}&appid={}",
        api.get_key()?,
        steamid,
        ugcid,
        appid
    );
    let data = api
        .request(&format!(
            "ISteamRemoteStorage/GetUGCFileDetails/v1/{}",
            &query
        ))
        .await?;
    return get_ugc_file_details::Response::from(&data);
}

pub async fn get_published_file_details(
    api: &mut crate::WebApi,
    itemcount: u32,
    publishedfileids_0: u64,
) -> Result<get_published_file_details::Response> {
    let query = format!(
        "?key={}&itemcount={}&publishedfileids[0]={}",
        api.get_key()?,
        itemcount,
        publishedfileids_0
    );
    let data = api
        .request(&format!(
            "ISteamRemoteStorage/GetPublishedFileDetails/v1/{}",
            &query
        ))
        .await?;
    return get_published_file_details::Response::from(&data);
}

pub async fn get_collection_details(
    api: &mut crate::WebApi,
    collectioncount: u32,
    publishedfileids_0: u64,
) -> Result<get_collection_details::Response> {
    let query = format!(
        "?key={}&collectioncount={}&publishedfileids[0]={}",
        api.get_key()?,
        collectioncount,
        publishedfileids_0
    );
    let data = api
        .request(&format!(
            "ISteamRemoteStorage/GetCollectionDetails/v1/{}",
            &query
        ))
        .await?;
    return get_collection_details::Response::from(&data);
}
