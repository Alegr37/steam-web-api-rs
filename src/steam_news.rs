use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_news_for_app;

pub async fn get_news_for_app(
    api: &mut crate::WebApi,
    appid: u32,
    maxlength: Option<u32>,
    enddate: Option<u32>,
    count: Option<u32>,
    feeds: Option<&str>,
    tags: Option<&str>,
) -> Result<get_news_for_app::Response> {
    let query = format!(
        "?key={}&appid={}&maxlength={}&enddate={}&count={}&feeds={}&tags={}",
        api.get_key()?,
        appid,
        maxlength.unwrap_or_default(),
        enddate.unwrap_or_default(),
        count.unwrap_or_default(),
        feeds.unwrap_or_default(),
        tags.unwrap_or_default()
    );
    let data = api
        .request(&format!("ISteamNews/GetNewsForApp/v2/{}", &query))
        .await?;
    return get_news_for_app::Response::from(&data);
}
