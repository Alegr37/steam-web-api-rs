use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod report_event;

pub struct ClientStatsDotaUnderlords<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> ClientStatsDotaUnderlords<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn report_event(&'a mut self) -> Result<report_event::Response> {
        let query = format!("?key={}", self.api.get_key()?);
        let data = self
            .api
            .request(&format!("IClientStats_1046930/ReportEvent/v1/{}", &query))
            .await?;
        return report_event::Response::from(&data);
    }
}
