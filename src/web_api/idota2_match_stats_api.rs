
use crate::web_api;

pub struct IDOTA2MatchStatsApi<'a> {
    web_api: &'a mut web_api::WebApi,
    app_id: web_api::AppId,
}

impl<'a> IDOTA2MatchStatsApi<'a> {
    pub fn new(web_api: &'a mut web_api::WebApi, app_id: web_api::AppId) -> Self {
        return Self{web_api, app_id}
    }
}

