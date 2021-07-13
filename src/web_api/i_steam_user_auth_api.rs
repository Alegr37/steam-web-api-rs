
use crate::web_api;

pub struct ISteamUserAuthApi<'a> {
    web_api: &'a mut web_api::WebApi,
}

impl<'a> ISteamUserAuthApi<'a> {
    pub fn new(web_api: &'a mut web_api::WebApi) -> Self {
        return Self{web_api}
    }
}

