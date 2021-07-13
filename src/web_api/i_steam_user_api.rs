
use crate::web_api;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

pub struct ISteamUserApi<'a> {
    web_api: &'a mut web_api::WebApi,
}

pub struct Friend {
    steamid: web_api::SteamId,
    relationship: Relationship,
    friend_since: DateTime<Utc>,
}

pub enum Relationship {
    Friend,
}

pub struct FriendList {
    friends: Vec<Friend>,
}

impl<'a> ISteamUserApi<'a> {
    pub fn new(web_api: &'a mut web_api::WebApi) -> Self {
        return Self{web_api}
    }

    pub fn GetFriendList(&'a self, id: web_api::SteamId) -> FriendList {
        unimplemented!();
    }
}

