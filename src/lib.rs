mod credentials;
mod web_api;
pub use web_api::{ApiKeyProvider, AppId, SteamId, WebApi, WebApiConfig, WebApiKey};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
