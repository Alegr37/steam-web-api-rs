
use crate::web_api;

pub struct ISteamWebUserPresenceOAuthApi<'a> {
    web_api: &'a mut web_api::WebApi,
}

impl<'a> ISteamWebUserPresenceOAuthApi<'a> {
    pub fn new(web_api: &'a mut web_api::WebApi) -> Self {
        return Self{web_api}
    }
}

