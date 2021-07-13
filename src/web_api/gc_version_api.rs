use crate::web_api;

pub struct IGCVersionApi<'a> {
    web_api: &'a mut web_api::WebApi,
    app_id: web_api::AppId,
}

impl<'a> IGCVersionApi<'a> {
    pub fn new(web_api: &'a mut web_api::WebApi, app_id: web_api::AppId) -> Self {
        return Self { web_api, app_id };
    }
}
