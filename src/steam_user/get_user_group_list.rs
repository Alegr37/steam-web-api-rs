use crate::common::*;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGroup {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    guid: GroupId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGroupListInerResponse {
    success: bool,
    groups: Vec<UserGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGroupListResponse {
    response: UserGroupListInerResponse,
}

pub struct Response {
    pub groups: Vec<UserGroup>,
}

impl Response {
    pub fn from(data: &[u8]) -> Result<Self> {
        let j: UserGroupListResponse = serde_json::from_slice(&data)?;
        if !j.response.success {
            bail!("unexpected success value: {}", j.response.success)
        }
        Ok(Response {
            groups: j.response.groups,
        })
    }
}
