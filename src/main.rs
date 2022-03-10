#![deny(warnings)]

use clap::Parser;
use log::LevelFilter;

use crate::command::{TbaCommand, TbaExtractor};
use crate::error::TbaError;

mod command;
pub(crate) mod data;
pub(crate) mod error;

fn main() -> Result<(), TbaError> {
    let args = TbaExtractor::parse();
    env_logger::Builder::new()
        .filter_level(match args.global_args.verbose {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        })
        .init();

    args.subcommand.run(args.global_args)
}
