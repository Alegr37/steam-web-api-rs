use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_golden_wrenches;

pub struct ItemsTeamFortress2<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> ItemsTeamFortress2<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_golden_wrenches(&'a mut self) -> Result<get_golden_wrenches::Response> {
        let query = format!("?key={}", self.api.get_key()?);
        let data = self
            .api
            .request(&format!("ITFItems_440/GetGoldenWrenches/v2/{}", &query))
            .await?;
        return get_golden_wrenches::Response::from(&data);
    }
}
