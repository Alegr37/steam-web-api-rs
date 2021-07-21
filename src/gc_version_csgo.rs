use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_server_version;

pub async fn get_server_version(api: &mut crate::WebApi) -> Result<get_server_version::Response> {
    let query = format!("?key={}", api.get_key()?);
    let data = api
        .request(&format!("IGCVersion_730/GetServerVersion/v1/{}", &query))
        .await?;
    return get_server_version::Response::from(&data);
}
