use clap::Args;
use serde::Serialize;

use crate::command::global_args::GlobalArgs;
use crate::command::http::{TbaReqwestExt, TBA_URL};
use crate::data::_match::{simple_team_breakdown, Match};
use crate::data::score_breakdown_2022::ScoreBreakdown2022;
use crate::{TbaCommand, TbaError};

#[derive(Args, Debug)]
#[clap(about = "Get data about if each team taxied in auto in a match")]
pub struct AutoTaxi {
    event_key: String,
}

impl TbaCommand for AutoTaxi {
    fn run(self, mut global_args: GlobalArgs) -> Result<(), TbaError> {
        let http_client = global_args.http_client()?;

        let event_key = &self.event_key;
        let matches: Vec<Match<ScoreBreakdown2022>> = http_client
            .get(format!("{TBA_URL}/event/{event_key}/matches"))
            .tba_send_for_json()?;

        let mut rows: Vec<AutoTaxiRow> = simple_team_breakdown(matches.into_iter())
            .map(|sb| AutoTaxiRow {
                match_number: sb.match_number,
                team: sb.team_number,
                auto_taxi: sb.score_breakdown.taxi[sb.team_index],
            })
            .collect();

        rows.sort_by_key(|row| [row.match_number, row.team]);

        global_args.write_csv(&rows)?;

        Ok(())
    }
}

#[derive(Serialize)]
struct AutoTaxiRow {
    team: u32,
    match_number: u32,
    auto_taxi: bool,
}
