use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_server_version;

pub struct GcVersionCsgo<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> GcVersionCsgo<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_server_version(&'a mut self) -> Result<get_server_version::Response> {
        let query = format!("?key={}", self.api.get_key()?);
        let data = self
            .api
            .request(&format!("IGCVersion_730/GetServerVersion/v1/{}", &query))
            .await?;
        return get_server_version::Response::from(&data);
    }
}
