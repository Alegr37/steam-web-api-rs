
use crate::web_api;

pub struct IEconDOTA2Api<'a> {
    web_api: &'a mut web_api::WebApi,
    app_id: web_api::AppId,
}

impl<'a> IEconDOTA2Api<'a> {
    pub fn new(web_api: &'a mut web_api::WebApi, app_id: web_api::AppId) -> Self {
        return Self{web_api, app_id}
    }
}

