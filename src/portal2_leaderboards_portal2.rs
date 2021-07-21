use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_bucketized_data;

pub async fn get_bucketized_data(
    api: &mut crate::WebApi,
    leaderboard_name: &str,
) -> Result<get_bucketized_data::Response> {
    let query = format!(
        "?key={}&leaderboardName={}",
        api.get_key()?,
        leaderboard_name
    );
    let data = api
        .request(&format!(
            "IPortal2Leaderboards_620/GetBucketizedData/v1/{}",
            &query
        ))
        .await?;
    return get_bucketized_data::Response::from(&data);
}
