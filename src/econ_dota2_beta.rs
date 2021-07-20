use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_event_stats_for_account;
pub mod get_game_items;
pub mod get_heroes;
pub mod get_item_icon_path;
pub mod get_rarities;
pub mod get_tournament_prize_pool;

pub struct EconDota2Beta<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> EconDota2Beta<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_tournament_prize_pool(
        &'a mut self,
        leagueid: Option<u32>,
    ) -> Result<get_tournament_prize_pool::Response> {
        let query = format!(
            "?key={}&leagueid={}",
            self.api.get_key()?,
            leagueid.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!(
                "IEconDOTA2_205790/GetTournamentPrizePool/v1/{}",
                &query
            ))
            .await?;
        return get_tournament_prize_pool::Response::from(&data);
    }

    pub async fn get_rarities(
        &'a mut self,
        language: Option<&str>,
    ) -> Result<get_rarities::Response> {
        let query = format!(
            "?key={}&language={}",
            self.api.get_key()?,
            language.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("IEconDOTA2_205790/GetRarities/v1/{}", &query))
            .await?;
        return get_rarities::Response::from(&data);
    }

    pub async fn get_item_icon_path(
        &'a mut self,
        iconname: &str,
        icontype: Option<u32>,
    ) -> Result<get_item_icon_path::Response> {
        let query = format!(
            "?key={}&iconname={}&icontype={}",
            self.api.get_key()?,
            iconname,
            icontype.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("IEconDOTA2_205790/GetItemIconPath/v1/{}", &query))
            .await?;
        return get_item_icon_path::Response::from(&data);
    }

    pub async fn get_heroes(
        &'a mut self,
        language: Option<&str>,
        itemizedonly: Option<bool>,
    ) -> Result<get_heroes::Response> {
        let query = format!(
            "?key={}&language={}&itemizedonly={}",
            self.api.get_key()?,
            language.unwrap_or_default(),
            itemizedonly.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("IEconDOTA2_205790/GetHeroes/v1/{}", &query))
            .await?;
        return get_heroes::Response::from(&data);
    }

    pub async fn get_game_items(
        &'a mut self,
        language: Option<&str>,
    ) -> Result<get_game_items::Response> {
        let query = format!(
            "?key={}&language={}",
            self.api.get_key()?,
            language.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!("IEconDOTA2_205790/GetGameItems/v1/{}", &query))
            .await?;
        return get_game_items::Response::from(&data);
    }

    pub async fn get_event_stats_for_account(
        &'a mut self,
        eventid: u32,
        accountid: u32,
        language: Option<&str>,
    ) -> Result<get_event_stats_for_account::Response> {
        let query = format!(
            "?key={}&eventid={}&accountid={}&language={}",
            self.api.get_key()?,
            eventid,
            accountid,
            language.unwrap_or_default()
        );
        let data = self
            .api
            .request(&format!(
                "IEconDOTA2_205790/GetEventStatsForAccount/v1/{}",
                &query
            ))
            .await?;
        return get_event_stats_for_account::Response::from(&data);
    }
}
