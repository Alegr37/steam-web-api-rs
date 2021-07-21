use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
pub mod report_cheat_data;

pub async fn report_cheat_data(
    api: &mut crate::WebApi,
    steamid: crate::SteamId,
    appid: u32,
    pathandfilename: &str,
    webcheaturl: &str,
    time_now: u64,
    time_started: u64,
    time_stopped: u64,
    cheatname: &str,
    game_process_id: u32,
    cheat_process_id: u32,
    cheat_param_1: u64,
    cheat_param_2: u64,
    cheat_data_dump: &str,
) -> Result<report_cheat_data::Response> {
    let query = format!("?key={}&steamid={}&appid={}&pathandfilename={}&webcheaturl={}&time_now={}&time_started={}&time_stopped={}&cheatname={}&game_process_id={}&cheat_process_id={}&cheat_param_1={}&cheat_param_2={}&cheat_data_dump={}", api.get_key()?, steamid, appid, pathandfilename, webcheaturl, time_now, time_started, time_stopped, cheatname, game_process_id, cheat_process_id, cheat_param_1, cheat_param_2, cheat_data_dump);
    let data = api
        .request(&format!(
            "ICheatReportingService/ReportCheatData/v1/{}",
            &query
        ))
        .await?;
    return report_cheat_data::Response::from(&data);
}
