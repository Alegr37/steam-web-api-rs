use anyhow::Result;
use std::str;

// TODO: add real Response struct
pub struct Response {}

impl Response {
    pub fn from(data: &[u8]) -> Result<Self> {
        if let Ok(s) = str::from_utf8(data) {
            unimplemented!("{}", s);
        }
        unimplemented!("{:?}", data)
    }
}
