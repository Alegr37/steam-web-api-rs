use tracing::{error};

#[derive(Debug)]
pub struct AuthError;

#[derive(Debug)]
struct WebApiKey(String);

pub enum CredentialsConfig {
    ApiKey(WebApiKey),
    ApiKeyList(Vec<WebApiKey>),
}

pub trait ApiKeyProvider {
    fn get_key(&self) -> Option<&WebApiKey>;
    fn onAuthError(&mut self, key: &WebApiKey, err: &AuthError);
}

struct SingleApiKey {
    key: WebApiKey,
}

impl ApiKeyProvider for SingleApiKey {
    fn get_key(&self) -> Option<&WebApiKey> {
        return Some(&self.key);
    }
    fn onAuthError(&mut self, key: &WebApiKey, error: &AuthError) {
        error!("auth error occurred with {:?}: {:?}", key, error)
    }
}

pub fn new(config: CredentialsConfig) -> Box<dyn ApiKeyProvider> {
    match config {
        CredentialsConfig::ApiKey(key) => return Box::new(SingleApiKey{key}),
        _ => unimplemented!(),
    }
}
