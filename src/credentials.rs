use anyhow::Result;
use tracing::error;

pub use String as WebApiKey;

pub trait ApiKeyProvider {
    fn get_key(&self) -> Result<&WebApiKey>;
    fn on_auth_error(&mut self, key: &WebApiKey);
}

struct SingleApiKey {
    key: WebApiKey,
}

impl ApiKeyProvider for SingleApiKey {
    fn get_key(&self) -> Result<&WebApiKey> {
        return Ok(&self.key);
    }
    fn on_auth_error(&mut self, key: &WebApiKey) {
        error!("auth error occurred with {}", key)
    }
}

impl From<WebApiKey> for Box<dyn ApiKeyProvider> {
    fn from(key: WebApiKey) -> Self {
        return Box::new(SingleApiKey { key: key.clone() });
    }
}
