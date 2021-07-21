use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_golden_wrenches;

pub async fn get_golden_wrenches(api: &mut crate::WebApi) -> Result<get_golden_wrenches::Response> {
    let query = format!("?key={}", api.get_key()?);
    let data = api
        .request(&format!("ITFItems_440/GetGoldenWrenches/v2/{}", &query))
        .await?;
    return get_golden_wrenches::Response::from(&data);
}
