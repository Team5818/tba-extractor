use serde::Deserialize;

#[derive(Deserialize)]
pub struct Alliance {
    surrogate_team_keys: Vec<String>,
    team_keys: Vec<String>,
}

impl Alliance {
    pub fn simple_team_keys(&self) -> [u32; 3] {
        // I can't be bothered to check this...
        assert!(
            self.surrogate_team_keys.is_empty(),
            "Surrogate team keys cannot contain values: {:?}",
            self.surrogate_team_keys
        );
        assert_eq!(
            self.team_keys.len(),
            3,
            "Need exactly 3 team keys: {:?}",
            self.team_keys
        );
        [0, 1, 2].map(|i| &self.team_keys[i]).map(|s| {
            s.strip_prefix("frc")
                .expect(&*format!("team key doesn't start with frc? {s}"))
                .parse::<u32>()
                .expect(&*format!("team key isn't a number? {s}"))
        })
    }
}

/// Helper struct for fields that come in a blue and red pair.
///
/// Iteration occurs over blue, then red.
#[derive(Deserialize)]
pub struct ByAlliance<SB> {
    blue: SB,
    red: SB,
}

impl<SB> IntoIterator for ByAlliance<SB> {
    type Item = SB;
    type IntoIter = <[SB; 2] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        [self.blue, self.red].into_iter()
    }
}
