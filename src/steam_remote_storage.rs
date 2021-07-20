use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_collection_details;
pub mod get_published_file_details;
pub mod get_ugc_file_details;

pub struct SteamRemoteStorage<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> SteamRemoteStorage<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_ugc_file_details(
        &'a mut self,
        steamid: crate::SteamId,
        ugcid: u64,
        appid: u32,
    ) -> Result<get_ugc_file_details::Response> {
        let query = format!(
            "?key={}&steamid={}&ugcid={}&appid={}",
            self.api.get_key()?,
            steamid,
            ugcid,
            appid
        );
        let data = self
            .api
            .request(&format!(
                "ISteamRemoteStorage/GetUGCFileDetails/v1/{}",
                &query
            ))
            .await?;
        return get_ugc_file_details::Response::from(&data);
    }

    pub async fn get_published_file_details(
        &'a mut self,
        itemcount: u32,
        publishedfileids_0: u64,
    ) -> Result<get_published_file_details::Response> {
        let query = format!(
            "?key={}&itemcount={}&publishedfileids[0]={}",
            self.api.get_key()?,
            itemcount,
            publishedfileids_0
        );
        let data = self
            .api
            .request(&format!(
                "ISteamRemoteStorage/GetPublishedFileDetails/v1/{}",
                &query
            ))
            .await?;
        return get_published_file_details::Response::from(&data);
    }

    pub async fn get_collection_details(
        &'a mut self,
        collectioncount: u32,
        publishedfileids_0: u64,
    ) -> Result<get_collection_details::Response> {
        let query = format!(
            "?key={}&collectioncount={}&publishedfileids[0]={}",
            self.api.get_key()?,
            collectioncount,
            publishedfileids_0
        );
        let data = self
            .api
            .request(&format!(
                "ISteamRemoteStorage/GetCollectionDetails/v1/{}",
                &query
            ))
            .await?;
        return get_collection_details::Response::from(&data);
    }
}
