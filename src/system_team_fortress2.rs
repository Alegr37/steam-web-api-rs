use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_world_status;

pub struct SystemTeamFortress2<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> SystemTeamFortress2<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_world_status(&'a mut self) -> Result<get_world_status::Response> {
        let query = format!("?key={}", self.api.get_key()?);
        let data = self
            .api
            .request(&format!("ITFSystem_440/GetWorldStatus/v1/{}", &query))
            .await?;
        return get_world_status::Response::from(&data);
    }
}
