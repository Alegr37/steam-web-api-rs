use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_details;
pub mod get_user_files;
pub mod get_user_vote_summary;
pub mod query_files;

pub async fn query_files(
    api: &mut crate::WebApi,
    query_type: u32,
    page: u32,
    cursor: &str,
    numperpage: Option<u32>,
    creator_appid: u32,
    appid: u32,
    requiredtags: &str,
    excludedtags: &str,
    match_all_tags: Option<bool>,
    required_flags: &str,
    omitted_flags: &str,
    search_text: &str,
    filetype: u32,
    child_publishedfileid: u64,
    days: u32,
    include_recent_votes_only: bool,
    cache_max_age_seconds: Option<u32>,
    language: Option<i32>,
    required_kv_tags: String,
    taggroups: String,
    totalonly: bool,
    ids_only: bool,
    return_vote_data: bool,
    return_tags: bool,
    return_kv_tags: bool,
    return_previews: bool,
    return_children: bool,
    return_short_description: bool,
    return_for_sale_data: bool,
    return_metadata: Option<bool>,
    return_playtime_stats: u32,
    return_details: bool,
    strip_description_bbcode: bool,
    desired_revision: Option<String>,
    return_reactions: Option<bool>,
) -> Result<query_files::Response> {
    let query = format!("?key={}&query_type={}&page={}&cursor={}&numperpage={}&creator_appid={}&appid={}&requiredtags={}&excludedtags={}&match_all_tags={}&required_flags={}&omitted_flags={}&search_text={}&filetype={}&child_publishedfileid={}&days={}&include_recent_votes_only={}&cache_max_age_seconds={}&language={}&required_kv_tags={}&taggroups={}&totalonly={}&ids_only={}&return_vote_data={}&return_tags={}&return_kv_tags={}&return_previews={}&return_children={}&return_short_description={}&return_for_sale_data={}&return_metadata={}&return_playtime_stats={}&return_details={}&strip_description_bbcode={}&desired_revision={}&return_reactions={}", api.get_key()?, query_type, page, cursor, numperpage.unwrap_or_default(), creator_appid, appid, requiredtags, excludedtags, match_all_tags.unwrap_or_default(), required_flags, omitted_flags, search_text, filetype, child_publishedfileid, days, include_recent_votes_only, cache_max_age_seconds.unwrap_or_default(), language.unwrap_or_default(), required_kv_tags, taggroups, totalonly, ids_only, return_vote_data, return_tags, return_kv_tags, return_previews, return_children, return_short_description, return_for_sale_data, return_metadata.unwrap_or_default(), return_playtime_stats, return_details, strip_description_bbcode, desired_revision.unwrap_or_default(), return_reactions.unwrap_or_default());
    let data = api
        .request(&format!("IPublishedFileService/QueryFiles/v1/{}", &query))
        .await?;
    return query_files::Response::from(&data);
}

pub async fn get_user_vote_summary(
    api: &mut crate::WebApi,
    publishedfileids: u64,
) -> Result<get_user_vote_summary::Response> {
    let query = format!(
        "?key={}&publishedfileids={}",
        api.get_key()?,
        publishedfileids
    );
    let data = api
        .request(&format!(
            "IPublishedFileService/GetUserVoteSummary/v1/{}",
            &query
        ))
        .await?;
    return get_user_vote_summary::Response::from(&data);
}

pub async fn get_user_files(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    appid: u32,
    page: Option<u32>,
    numperpage: Option<u32>,
    rtype: Option<&str>,
    sortmethod: Option<&str>,
    privacy: u32,
    requiredtags: &str,
    excludedtags: &str,
    required_kv_tags: String,
    filetype: u32,
    creator_appid: u32,
    match_cloud_filename: &str,
    cache_max_age_seconds: Option<u32>,
    language: Option<i32>,
    taggroups: String,
    totalonly: bool,
    ids_only: bool,
    return_vote_data: Option<bool>,
    return_tags: bool,
    return_kv_tags: Option<bool>,
    return_previews: bool,
    return_children: bool,
    return_short_description: Option<bool>,
    return_for_sale_data: bool,
    return_metadata: Option<bool>,
    return_playtime_stats: u32,
    strip_description_bbcode: bool,
    return_reactions: Option<bool>,
    startindex_override: u32,
    desired_revision: Option<String>,
) -> Result<get_user_files::Response> {
    let query = format!("?key={}&steamid={}&appid={}&page={}&numperpage={}&type={}&sortmethod={}&privacy={}&requiredtags={}&excludedtags={}&required_kv_tags={}&filetype={}&creator_appid={}&match_cloud_filename={}&cache_max_age_seconds={}&language={}&taggroups={}&totalonly={}&ids_only={}&return_vote_data={}&return_tags={}&return_kv_tags={}&return_previews={}&return_children={}&return_short_description={}&return_for_sale_data={}&return_metadata={}&return_playtime_stats={}&strip_description_bbcode={}&return_reactions={}&startindex_override={}&desired_revision={}", api.get_key()?, steamid, appid, page.unwrap_or_default(), numperpage.unwrap_or_default(), rtype.unwrap_or_default(), sortmethod.unwrap_or_default(), privacy, requiredtags, excludedtags, required_kv_tags, filetype, creator_appid, match_cloud_filename, cache_max_age_seconds.unwrap_or_default(), language.unwrap_or_default(), taggroups, totalonly, ids_only, return_vote_data.unwrap_or_default(), return_tags, return_kv_tags.unwrap_or_default(), return_previews, return_children, return_short_description.unwrap_or_default(), return_for_sale_data, return_metadata.unwrap_or_default(), return_playtime_stats, strip_description_bbcode, return_reactions.unwrap_or_default(), startindex_override, desired_revision.unwrap_or_default());
    let data = api
        .request(&format!("IPublishedFileService/GetUserFiles/v1/{}", &query))
        .await?;
    return get_user_files::Response::from(&data);
}

pub async fn get_details(
    api: &mut crate::WebApi,
    publishedfileids: u64,
    includetags: bool,
    includeadditionalpreviews: bool,
    includechildren: bool,
    includekvtags: bool,
    includevotes: bool,
    short_description: bool,
    includeforsaledata: bool,
    includemetadata: bool,
    language: Option<i32>,
    return_playtime_stats: u32,
    appid: u32,
    strip_description_bbcode: bool,
    desired_revision: Option<String>,
    includereactions: Option<bool>,
) -> Result<get_details::Response> {
    let query = format!("?key={}&publishedfileids={}&includetags={}&includeadditionalpreviews={}&includechildren={}&includekvtags={}&includevotes={}&short_description={}&includeforsaledata={}&includemetadata={}&language={}&return_playtime_stats={}&appid={}&strip_description_bbcode={}&desired_revision={}&includereactions={}", api.get_key()?, publishedfileids, includetags, includeadditionalpreviews, includechildren, includekvtags, includevotes, short_description, includeforsaledata, includemetadata, language.unwrap_or_default(), return_playtime_stats, appid, strip_description_bbcode, desired_revision.unwrap_or_default(), includereactions.unwrap_or_default());
    let data = api
        .request(&format!("IPublishedFileService/GetDetails/v1/{}", &query))
        .await?;
    return get_details::Response::from(&data);
}
