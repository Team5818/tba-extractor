use serde::{Deserialize, Deserializer};

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreBreakdown2022 {
    #[serde(flatten, deserialize_with = "intermediate_taxi")]
    pub taxi: [bool; 3],
    #[serde(flatten, deserialize_with = "intermediate_endgame_state")]
    pub endgames: [RobotEndgameState; 3],
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IntermediateEndgameState {
    endgame_robot_1: RobotEndgameState,
    endgame_robot_2: RobotEndgameState,
    endgame_robot_3: RobotEndgameState,
}

fn intermediate_endgame_state<'de, D>(de: D) -> Result<[RobotEndgameState; 3], D::Error>
where
    D: Deserializer<'de>,
{
    let base = IntermediateEndgameState::deserialize(de)?;
    Ok([
        base.endgame_robot_1,
        base.endgame_robot_2,
        base.endgame_robot_3,
    ])
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IntermediateTaxi {
    taxi_robot_1: RobotTaxi,
    taxi_robot_2: RobotTaxi,
    taxi_robot_3: RobotTaxi,
}

fn intermediate_taxi<'de, D>(de: D) -> Result<[bool; 3], D::Error>
where
    D: Deserializer<'de>,
{
    let base = IntermediateTaxi::deserialize(de)?;
    Ok([
        base.taxi_robot_1 == RobotTaxi::Yes,
        base.taxi_robot_2 == RobotTaxi::Yes,
        base.taxi_robot_3 == RobotTaxi::Yes,
    ])
}

#[derive(Clone, Copy, Deserialize)]
pub enum RobotEndgameState {
    None,
    Low,
    Mid,
    High,
    Traversal,
}

#[derive(Eq, PartialEq, Clone, Copy, Deserialize)]
pub enum RobotTaxi {
    No,
    Yes,
}
