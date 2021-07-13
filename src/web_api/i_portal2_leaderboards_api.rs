
use crate::web_api;

pub struct IPortal2LeaderboardsApi<'a> {
    web_api: &'a mut web_api::WebApi,
}

impl<'a> IPortal2LeaderboardsApi<'a> {
    pub fn new(web_api: &'a mut web_api::WebApi) -> Self {
        return Self{web_api}
    }
}

