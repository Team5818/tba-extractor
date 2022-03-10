use serde::Deserialize;

use crate::data::alliance::{Alliance, ByAlliance};

pub fn simple_team_breakdown<SB, I>(matches: I) -> impl Iterator<Item = SimpleBreakdown<SB>>
where
    SB: Clone,
    I: Iterator<Item = Match<SB>>,
{
    matches
        .filter(|m| m.comp_level == MatchCompType::Qualification)
        .flat_map(
            |Match {
                 alliances,
                 match_number,
                 score_breakdown,
                 ..
             }| {
                alliances
                    .into_iter()
                    .zip(score_breakdown.into_iter())
                    .flat_map(move |(alliance, sb)| {
                        let team_keys = alliance.simple_team_keys();
                        [0, 1, 2]
                            .map(|i| SimpleBreakdown {
                                match_number,
                                team_number: team_keys[i],
                                team_index: i,
                                score_breakdown: sb.clone(),
                            })
                            .into_iter()
                    })
            },
        )
}

pub struct SimpleBreakdown<SB> {
    pub match_number: u32,
    pub team_number: u32,
    pub team_index: usize,
    pub score_breakdown: SB,
}

#[derive(Deserialize)]
pub struct Match<SB> {
    pub alliances: ByAlliance<Alliance>,
    pub comp_level: MatchCompType,
    pub match_number: u32,
    pub score_breakdown: ByAlliance<SB>,
}

#[derive(Eq, PartialEq, Deserialize)]
pub enum MatchCompType {
    #[serde(rename = "qm")]
    Qualification,
    /// I have no idea what "ef" is supposed to be.
    #[serde(rename = "ef")]
    IfYouSeeThisValuePleaseTellMeWhatItMeans,
    #[serde(rename = "qf")]
    QuarterFinal,
    #[serde(rename = "sf")]
    SemiFinal,
    #[serde(rename = "f")]
    Final,
}
