use crate::common::*;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResolvedUrl {
    steamid: Option<String>,
    message: Option<String>,
    success: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResolvedUrlResponse {
    response: ResolvedUrl,
}

pub struct Response {
    pub steamid: SteamId,
}

impl Response {
    pub fn from(data: &[u8]) -> Result<Self> {
        let j: ResolvedUrlResponse = serde_json::from_slice(&data)?;
        if let Some(id) = j.response.steamid {
            let id = id.parse::<SteamId>()?;
            return Ok(Response { steamid: id });
        }
        if let Some(m) = j.response.message {
            bail!("error_code: {}, message: {:?}", j.response.success, m)
        }
        bail!("error_code: {}", j.response.success)
    }
}
