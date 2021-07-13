
use crate::web_api;

pub struct ISteamOAuth2Api<'a> {
    web_api: &'a mut web_api::WebApi,
}

impl<'a> ISteamOAuth2Api<'a> {
    pub fn new(web_api: &'a mut web_api::WebApi) -> Self {
        return Self{web_api}
    }
}

