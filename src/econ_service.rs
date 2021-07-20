use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod cancel_trade_offer;
pub mod decline_trade_offer;
pub mod get_trade_history;
pub mod get_trade_hold_durations;
pub mod get_trade_offer;
pub mod get_trade_offers;
pub mod get_trade_offers_summary;
pub mod get_trade_status;

pub struct EconService<'a> {
    api: &'a mut crate::WebApi,
}

impl<'a> EconService<'a> {
    pub fn new(api: &'a mut crate::WebApi) -> Self {
        return Self { api };
    }

    pub async fn get_trade_status(
        &'a mut self,
        tradeid: u64,
        get_descriptions: bool,
        language: &str,
    ) -> Result<get_trade_status::Response> {
        let query = format!(
            "?key={}&tradeid={}&get_descriptions={}&language={}",
            self.api.get_key()?,
            tradeid,
            get_descriptions,
            language
        );
        let data = self
            .api
            .request(&format!("IEconService/GetTradeStatus/v1/{}", &query))
            .await?;
        return get_trade_status::Response::from(&data);
    }

    pub async fn get_trade_offers_summary(
        &'a mut self,
        time_last_visit: u32,
    ) -> Result<get_trade_offers_summary::Response> {
        let query = format!(
            "?key={}&time_last_visit={}",
            self.api.get_key()?,
            time_last_visit
        );
        let data = self
            .api
            .request(&format!("IEconService/GetTradeOffersSummary/v1/{}", &query))
            .await?;
        return get_trade_offers_summary::Response::from(&data);
    }

    pub async fn get_trade_offers(
        &'a mut self,
        get_sent_offers: bool,
        get_received_offers: bool,
        get_descriptions: bool,
        language: &str,
        active_only: bool,
        historical_only: bool,
        time_historical_cutoff: u32,
        cursor: Option<u32>,
    ) -> Result<get_trade_offers::Response> {
        let query = format!("?key={}&get_sent_offers={}&get_received_offers={}&get_descriptions={}&language={}&active_only={}&historical_only={}&time_historical_cutoff={}&cursor={}", self.api.get_key()?, get_sent_offers, get_received_offers, get_descriptions, language, active_only, historical_only, time_historical_cutoff, cursor.unwrap_or_default());
        let data = self
            .api
            .request(&format!("IEconService/GetTradeOffers/v1/{}", &query))
            .await?;
        return get_trade_offers::Response::from(&data);
    }

    pub async fn get_trade_offer(
        &'a mut self,
        tradeofferid: u64,
        language: &str,
        get_descriptions: bool,
    ) -> Result<get_trade_offer::Response> {
        let query = format!(
            "?key={}&tradeofferid={}&language={}&get_descriptions={}",
            self.api.get_key()?,
            tradeofferid,
            language,
            get_descriptions
        );
        let data = self
            .api
            .request(&format!("IEconService/GetTradeOffer/v1/{}", &query))
            .await?;
        return get_trade_offer::Response::from(&data);
    }

    pub async fn get_trade_hold_durations(
        &'a mut self,
        steamid_target: u64,
        trade_offer_access_token: &str,
    ) -> Result<get_trade_hold_durations::Response> {
        let query = format!(
            "?key={}&steamid_target={}&trade_offer_access_token={}",
            self.api.get_key()?,
            steamid_target,
            trade_offer_access_token
        );
        let data = self
            .api
            .request(&format!("IEconService/GetTradeHoldDurations/v1/{}", &query))
            .await?;
        return get_trade_hold_durations::Response::from(&data);
    }

    pub async fn get_trade_history(
        &'a mut self,
        max_trades: u32,
        start_after_time: u32,
        start_after_tradeid: u64,
        navigating_back: bool,
        get_descriptions: bool,
        language: &str,
        include_failed: bool,
        include_total: bool,
    ) -> Result<get_trade_history::Response> {
        let query = format!("?key={}&max_trades={}&start_after_time={}&start_after_tradeid={}&navigating_back={}&get_descriptions={}&language={}&include_failed={}&include_total={}", self.api.get_key()?, max_trades, start_after_time, start_after_tradeid, navigating_back, get_descriptions, language, include_failed, include_total);
        let data = self
            .api
            .request(&format!("IEconService/GetTradeHistory/v1/{}", &query))
            .await?;
        return get_trade_history::Response::from(&data);
    }

    pub async fn decline_trade_offer(
        &'a mut self,
        tradeofferid: u64,
    ) -> Result<decline_trade_offer::Response> {
        let query = format!("?key={}&tradeofferid={}", self.api.get_key()?, tradeofferid);
        let data = self
            .api
            .request(&format!("IEconService/DeclineTradeOffer/v1/{}", &query))
            .await?;
        return decline_trade_offer::Response::from(&data);
    }

    pub async fn cancel_trade_offer(
        &'a mut self,
        tradeofferid: u64,
    ) -> Result<cancel_trade_offer::Response> {
        let query = format!("?key={}&tradeofferid={}", self.api.get_key()?, tradeofferid);
        let data = self
            .api
            .request(&format!("IEconService/CancelTradeOffer/v1/{}", &query))
            .await?;
        return cancel_trade_offer::Response::from(&data);
    }
}
