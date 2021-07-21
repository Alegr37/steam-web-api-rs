use crate::common::*;
use anyhow::Result;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Friend {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub steamid: SteamId,
    pub relationship: Relationship,
    #[serde(with = "ts_seconds")]
    pub friend_since: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Relationship {
    #[serde(rename = "friend")]
    Friend,
}

#[derive(Serialize, Deserialize, Debug)]
struct FriendList {
    friends: Vec<Friend>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FriendListResponse {
    friendslist: FriendList,
}

pub struct Response {
    pub friends: Vec<Friend>,
}

impl Response {
    pub fn from(data: &[u8]) -> Result<Self> {
        let j: FriendListResponse = serde_json::from_slice(&data)?;
        Ok(Response {
            friends: j.friendslist.friends,
        })
    }
}
