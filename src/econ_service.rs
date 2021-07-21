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

pub async fn get_trade_status(
    api: &mut crate::WebApi,
    tradeid: u64,
    get_descriptions: bool,
    language: &str,
) -> Result<get_trade_status::Response> {
    let query = format!(
        "?key={}&tradeid={}&get_descriptions={}&language={}",
        api.get_key()?,
        tradeid,
        get_descriptions,
        language
    );
    let data = api
        .request(&format!("IEconService/GetTradeStatus/v1/{}", &query))
        .await?;
    return get_trade_status::Response::from(&data);
}

pub async fn get_trade_offers_summary(
    api: &mut crate::WebApi,
    time_last_visit: u32,
) -> Result<get_trade_offers_summary::Response> {
    let query = format!(
        "?key={}&time_last_visit={}",
        api.get_key()?,
        time_last_visit
    );
    let data = api
        .request(&format!("IEconService/GetTradeOffersSummary/v1/{}", &query))
        .await?;
    return get_trade_offers_summary::Response::from(&data);
}

pub async fn get_trade_offers(
    api: &mut crate::WebApi,
    get_sent_offers: bool,
    get_received_offers: bool,
    get_descriptions: bool,
    language: &str,
    active_only: bool,
    historical_only: bool,
    time_historical_cutoff: u32,
    cursor: Option<u32>,
) -> Result<get_trade_offers::Response> {
    let query = format!("?key={}&get_sent_offers={}&get_received_offers={}&get_descriptions={}&language={}&active_only={}&historical_only={}&time_historical_cutoff={}&cursor={}", api.get_key()?, get_sent_offers, get_received_offers, get_descriptions, language, active_only, historical_only, time_historical_cutoff, cursor.unwrap_or_default());
    let data = api
        .request(&format!("IEconService/GetTradeOffers/v1/{}", &query))
        .await?;
    return get_trade_offers::Response::from(&data);
}

pub async fn get_trade_offer(
    api: &mut crate::WebApi,
    tradeofferid: u64,
    language: &str,
    get_descriptions: bool,
) -> Result<get_trade_offer::Response> {
    let query = format!(
        "?key={}&tradeofferid={}&language={}&get_descriptions={}",
        api.get_key()?,
        tradeofferid,
        language,
        get_descriptions
    );
    let data = api
        .request(&format!("IEconService/GetTradeOffer/v1/{}", &query))
        .await?;
    return get_trade_offer::Response::from(&data);
}

pub async fn get_trade_hold_durations(
    api: &mut crate::WebApi,
    steamid_target: u64,
    trade_offer_access_token: &str,
) -> Result<get_trade_hold_durations::Response> {
    let query = format!(
        "?key={}&steamid_target={}&trade_offer_access_token={}",
        api.get_key()?,
        steamid_target,
        trade_offer_access_token
    );
    let data = api
        .request(&format!("IEconService/GetTradeHoldDurations/v1/{}", &query))
        .await?;
    return get_trade_hold_durations::Response::from(&data);
}

pub async fn get_trade_history(
    api: &mut crate::WebApi,
    max_trades: u32,
    start_after_time: u32,
    start_after_tradeid: u64,
    navigating_back: bool,
    get_descriptions: bool,
    language: &str,
    include_failed: bool,
    include_total: bool,
) -> Result<get_trade_history::Response> {
    let query = format!("?key={}&max_trades={}&start_after_time={}&start_after_tradeid={}&navigating_back={}&get_descriptions={}&language={}&include_failed={}&include_total={}", api.get_key()?, max_trades, start_after_time, start_after_tradeid, navigating_back, get_descriptions, language, include_failed, include_total);
    let data = api
        .request(&format!("IEconService/GetTradeHistory/v1/{}", &query))
        .await?;
    return get_trade_history::Response::from(&data);
}

pub async fn decline_trade_offer(
    api: &mut crate::WebApi,
    tradeofferid: u64,
) -> Result<decline_trade_offer::Response> {
    let query = format!("?key={}&tradeofferid={}", api.get_key()?, tradeofferid);
    let data = api
        .request(&format!("IEconService/DeclineTradeOffer/v1/{}", &query))
        .await?;
    return decline_trade_offer::Response::from(&data);
}

pub async fn cancel_trade_offer(
    api: &mut crate::WebApi,
    tradeofferid: u64,
) -> Result<cancel_trade_offer::Response> {
    let query = format!("?key={}&tradeofferid={}", api.get_key()?, tradeofferid);
    let data = api
        .request(&format!("IEconService/CancelTradeOffer/v1/{}", &query))
        .await?;
    return cancel_trade_offer::Response::from(&data);
}
