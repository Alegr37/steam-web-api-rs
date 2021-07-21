use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod get_league_listing;
pub mod get_live_league_games;
pub mod get_match_details;
pub mod get_match_history;
pub mod get_match_history_by_sequence_num;
pub mod get_team_info_by_team_id;
pub mod get_top_live_event_game;
pub mod get_top_live_game;
pub mod get_top_weekend_tourney_games;
pub mod get_tournament_player_stats;

pub async fn get_tournament_player_stats(
    api: &mut crate::WebApi,
    account_id: &str,
    league_id: Option<&str>,
    hero_id: Option<&str>,
    time_frame: Option<&str>,
    match_id: Option<u64>,
    phase_id: Option<u32>,
) -> Result<get_tournament_player_stats::Response> {
    let query = format!(
        "?key={}&account_id={}&league_id={}&hero_id={}&time_frame={}&match_id={}&phase_id={}",
        api.get_key()?,
        account_id,
        league_id.unwrap_or_default(),
        hero_id.unwrap_or_default(),
        time_frame.unwrap_or_default(),
        match_id.unwrap_or_default(),
        phase_id.unwrap_or_default()
    );
    let data = api
        .request(&format!(
            "IDOTA2Match_205790/GetTournamentPlayerStats/v2/{}",
            &query
        ))
        .await?;
    return get_tournament_player_stats::Response::from(&data);
}

pub async fn get_top_weekend_tourney_games(
    api: &mut crate::WebApi,
    partner: i32,
    home_division: Option<i32>,
) -> Result<get_top_weekend_tourney_games::Response> {
    let query = format!(
        "?key={}&partner={}&home_division={}",
        api.get_key()?,
        partner,
        home_division.unwrap_or_default()
    );
    let data = api
        .request(&format!(
            "IDOTA2Match_205790/GetTopWeekendTourneyGames/v1/{}",
            &query
        ))
        .await?;
    return get_top_weekend_tourney_games::Response::from(&data);
}

pub async fn get_top_live_game(
    api: &mut crate::WebApi,
    partner: i32,
) -> Result<get_top_live_game::Response> {
    let query = format!("?key={}&partner={}", api.get_key()?, partner);
    let data = api
        .request(&format!("IDOTA2Match_205790/GetTopLiveGame/v1/{}", &query))
        .await?;
    return get_top_live_game::Response::from(&data);
}

pub async fn get_top_live_event_game(
    api: &mut crate::WebApi,
    partner: i32,
) -> Result<get_top_live_event_game::Response> {
    let query = format!("?key={}&partner={}", api.get_key()?, partner);
    let data = api
        .request(&format!(
            "IDOTA2Match_205790/GetTopLiveEventGame/v1/{}",
            &query
        ))
        .await?;
    return get_top_live_event_game::Response::from(&data);
}

pub async fn get_team_info_by_team_id(
    api: &mut crate::WebApi,
    start_at_team_id: Option<u64>,
    teams_requested: Option<u32>,
) -> Result<get_team_info_by_team_id::Response> {
    let query = format!(
        "?key={}&start_at_team_id={}&teams_requested={}",
        api.get_key()?,
        start_at_team_id.unwrap_or_default(),
        teams_requested.unwrap_or_default()
    );
    let data = api
        .request(&format!(
            "IDOTA2Match_205790/GetTeamInfoByTeamID/v1/{}",
            &query
        ))
        .await?;
    return get_team_info_by_team_id::Response::from(&data);
}

pub async fn get_match_history_by_sequence_num(
    api: &mut crate::WebApi,
    start_at_match_seq_num: Option<u64>,
    matches_requested: Option<u32>,
) -> Result<get_match_history_by_sequence_num::Response> {
    let query = format!(
        "?key={}&start_at_match_seq_num={}&matches_requested={}",
        api.get_key()?,
        start_at_match_seq_num.unwrap_or_default(),
        matches_requested.unwrap_or_default()
    );
    let data = api
        .request(&format!(
            "IDOTA2Match_205790/GetMatchHistoryBySequenceNum/v1/{}",
            &query
        ))
        .await?;
    return get_match_history_by_sequence_num::Response::from(&data);
}

pub async fn get_match_history(
    api: &mut crate::WebApi,
    hero_id: Option<u32>,
    game_mode: Option<u32>,
    skill: Option<u32>,
    min_players: Option<&str>,
    account_id: Option<&str>,
    league_id: Option<&str>,
    start_at_match_id: Option<u64>,
    matches_requested: Option<&str>,
) -> Result<get_match_history::Response> {
    let query = format!("?key={}&hero_id={}&game_mode={}&skill={}&min_players={}&account_id={}&league_id={}&start_at_match_id={}&matches_requested={}", api.get_key()?, hero_id.unwrap_or_default(), game_mode.unwrap_or_default(), skill.unwrap_or_default(), min_players.unwrap_or_default(), account_id.unwrap_or_default(), league_id.unwrap_or_default(), start_at_match_id.unwrap_or_default(), matches_requested.unwrap_or_default());
    let data = api
        .request(&format!("IDOTA2Match_205790/GetMatchHistory/v1/{}", &query))
        .await?;
    return get_match_history::Response::from(&data);
}

pub async fn get_match_details(
    api: &mut crate::WebApi,
    match_id: u64,
) -> Result<get_match_details::Response> {
    let query = format!("?key={}&match_id={}", api.get_key()?, match_id);
    let data = api
        .request(&format!("IDOTA2Match_205790/GetMatchDetails/v1/{}", &query))
        .await?;
    return get_match_details::Response::from(&data);
}

pub async fn get_live_league_games(
    api: &mut crate::WebApi,
    league_id: Option<u32>,
    match_id: Option<u64>,
) -> Result<get_live_league_games::Response> {
    let query = format!(
        "?key={}&league_id={}&match_id={}",
        api.get_key()?,
        league_id.unwrap_or_default(),
        match_id.unwrap_or_default()
    );
    let data = api
        .request(&format!(
            "IDOTA2Match_205790/GetLiveLeagueGames/v1/{}",
            &query
        ))
        .await?;
    return get_live_league_games::Response::from(&data);
}

pub async fn get_league_listing(api: &mut crate::WebApi) -> Result<get_league_listing::Response> {
    let query = format!("?key={}", api.get_key()?);
    let data = api
        .request(&format!(
            "IDOTA2Match_205790/GetLeagueListing/v1/{}",
            &query
        ))
        .await?;
    return get_league_listing::Response::from(&data);
}
