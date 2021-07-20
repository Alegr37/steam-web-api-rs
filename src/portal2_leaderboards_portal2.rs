use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_bucketized_data;

pub struct Portal2LeaderboardsPortal2<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> Portal2LeaderboardsPortal2<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_bucketized_data(
        &'a mut self,
        leaderboard_name: &str,
    ) -> Result<get_bucketized_data::Response> {
        let query = format!(
            "?key={}&leaderboardName={}",
            self.api.get_key()?,
            leaderboard_name
        );
        let data = self
            .api
            .request(&format!(
                "IPortal2Leaderboards_620/GetBucketizedData/v1/{}",
                &query
            ))
            .await?;
        return get_bucketized_data::Response::from(&data);
    }
}
