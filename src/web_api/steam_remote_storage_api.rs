use crate::web_api;

pub struct ISteamRemoteStorageApi<'a> {
    web_api: &'a mut web_api::WebApi,
}

impl<'a> ISteamRemoteStorageApi<'a> {
    pub fn new(web_api: &'a mut web_api::WebApi) -> Self {
        return Self { web_api };
    }
}
