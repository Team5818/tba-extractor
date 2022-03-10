use clap::{Parser, Subcommand};

use global_args::GlobalArgs;

use crate::error::TbaError;

mod _2022;
mod global_args;
mod http;

pub trait TbaCommand {
    fn run(self, global_args: GlobalArgs) -> Result<(), TbaError>;
}

#[derive(Parser, Debug)]
#[clap(name = "tba-extractor", about = "TBA Data Extractor", version)]
pub struct TbaExtractor {
    #[clap(flatten)]
    pub global_args: GlobalArgs,
    /// Thing to do.
    #[clap(subcommand)]
    pub subcommand: TbaCommandChoices,
}

#[derive(Subcommand, Debug)]
pub enum TbaCommandChoices {
    ClimbBar(_2022::ClimbBar),
    AutoTaxi(_2022::AutoTaxi),
}

impl TbaCommand for TbaCommandChoices {
    fn run(self, global_args: GlobalArgs) -> Result<(), TbaError> {
        match self {
            Self::ClimbBar(sc) => sc.run(global_args),
            Self::AutoTaxi(sc) => sc.run(global_args),
        }
    }
}
