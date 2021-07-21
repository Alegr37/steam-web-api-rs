use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_app_list;

pub async fn get_app_list(
    api: &mut crate::WebApi,
    if_modified_since: Option<u32>,
    have_description_language: Option<&str>,
    include_games: Option<bool>,
    include_dlc: Option<bool>,
    include_software: Option<bool>,
    include_videos: Option<bool>,
    include_hardware: Option<bool>,
    last_appid: Option<u32>,
    max_results: Option<u32>,
) -> Result<get_app_list::Response> {
    let query = format!("?key={}&if_modified_since={}&have_description_language={}&include_games={}&include_dlc={}&include_software={}&include_videos={}&include_hardware={}&last_appid={}&max_results={}", api.get_key()?, if_modified_since.unwrap_or_default(), have_description_language.unwrap_or_default(), include_games.unwrap_or_default(), include_dlc.unwrap_or_default(), include_software.unwrap_or_default(), include_videos.unwrap_or_default(), include_hardware.unwrap_or_default(), last_appid.unwrap_or_default(), max_results.unwrap_or_default());
    let data = api
        .request(&format!("IStoreService/GetAppList/v1/{}", &query))
        .await?;
    return get_app_list::Response::from(&data);
}
