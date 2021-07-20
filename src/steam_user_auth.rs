use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod authenticate_user;
pub mod authenticate_user_ticket;

pub struct SteamUserAuth<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> SteamUserAuth<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn authenticate_user_ticket(
        &'a mut self,
        appid: u32,
        ticket: &str,
    ) -> Result<authenticate_user_ticket::Response> {
        let query = format!(
            "?key={}&appid={}&ticket={}",
            self.api.get_key()?,
            appid,
            ticket
        );
        let data = self
            .api
            .request(&format!(
                "ISteamUserAuth/AuthenticateUserTicket/v1/{}",
                &query
            ))
            .await?;
        return authenticate_user_ticket::Response::from(&data);
    }

    pub async fn authenticate_user(
        &'a mut self,
        steamid: crate::SteamId,
        sessionkey: String,
        encrypted_loginkey: String,
    ) -> Result<authenticate_user::Response> {
        let query = format!(
            "?key={}&steamid={}&sessionkey={}&encrypted_loginkey={}",
            self.api.get_key()?,
            steamid,
            sessionkey,
            encrypted_loginkey
        );
        let data = self
            .api
            .request(&format!("ISteamUserAuth/AuthenticateUser/v1/{}", &query))
            .await?;
        return authenticate_user::Response::from(&data);
    }
}
