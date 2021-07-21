use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod report_event;

pub async fn report_event(api: &mut crate::WebApi) -> Result<report_event::Response> {
    let query = format!("?key={}", api.get_key()?);
    let data = api
        .request(&format!("IClientStats_1046930/ReportEvent/v1/{}", &query))
        .await?;
    return report_event::Response::from(&data);
}
