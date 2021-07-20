use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod user_create_session;
pub mod user_delete_session;
pub mod user_update_session;

pub struct GameNotificationsService<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> GameNotificationsService<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn user_update_session(
        &'a mut self,
        sessionid: u64,
        appid: u32,
        title: String,
        users: String,
        steamid: crate::SteamId,
    ) -> Result<user_update_session::Response> {
        let query = format!(
            "?key={}&sessionid={}&appid={}&title={}&users={}&steamid={}",
            self.api.get_key()?,
            sessionid,
            appid,
            title,
            users,
            steamid
        );
        let data = self
            .api
            .request(&format!(
                "IGameNotificationsService/UserUpdateSession/v1/{}",
                &query
            ))
            .await?;
        return user_update_session::Response::from(&data);
    }

    pub async fn user_delete_session(
        &'a mut self,
        sessionid: u64,
        appid: u32,
        steamid: crate::SteamId,
    ) -> Result<user_delete_session::Response> {
        let query = format!(
            "?key={}&sessionid={}&appid={}&steamid={}",
            self.api.get_key()?,
            sessionid,
            appid,
            steamid
        );
        let data = self
            .api
            .request(&format!(
                "IGameNotificationsService/UserDeleteSession/v1/{}",
                &query
            ))
            .await?;
        return user_delete_session::Response::from(&data);
    }

    pub async fn user_create_session(
        &'a mut self,
        appid: u32,
        context: u64,
        title: String,
        users: String,
        steamid: crate::SteamId,
    ) -> Result<user_create_session::Response> {
        let query = format!(
            "?key={}&appid={}&context={}&title={}&users={}&steamid={}",
            self.api.get_key()?,
            appid,
            context,
            title,
            users,
            steamid
        );
        let data = self
            .api
            .request(&format!(
                "IGameNotificationsService/UserCreateSession/v1/{}",
                &query
            ))
            .await?;
        return user_create_session::Response::from(&data);
    }
}
