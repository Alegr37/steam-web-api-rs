use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_world_status;

pub async fn get_world_status(api: &mut crate::WebApi) -> Result<get_world_status::Response> {
    let query = format!("?key={}", api.get_key()?);
    let data = api
        .request(&format!("ITFSystem_440/GetWorldStatus/v1/{}", &query))
        .await?;
    return get_world_status::Response::from(&data);
}
