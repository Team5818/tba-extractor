use std::convert::Infallible;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use clap::Args;
use serde::Serialize;

use crate::command::http::CachedHttpClient;
use crate::TbaError;

#[derive(Args, Debug)]
pub struct GlobalArgs {
    /// Verbosity level, repeat to increase.
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: usize,
    /// CSV output location, `-` for STDOUT.
    #[clap(short, long, default_value = "-")]
    output: CommandOutput,
    /// Your 'The Blue Alliance' API key. Recommended to be passed via environment variable to
    /// keep it secret from other accounts on your system.
    #[clap(required = true, short, long, env = "TBA_API_KEY")]
    api_key: String,
    #[clap(skip)]
    http_client: CachedHttpClient,
}

impl GlobalArgs {
    pub fn http_client(&mut self) -> Result<&mut reqwest::blocking::Client, TbaError> {
        self.http_client.get(&self.api_key)
    }

    pub fn write_csv<T: Serialize>(&self, rows: &[T]) -> Result<(), TbaError> {
        let mut csv_writer = csv::WriterBuilder::new().from_writer(match &self.output {
            CommandOutput::Stdin => {
                log::debug!("Writing to STDOUT");
                Box::new(std::io::stdout()) as Box<dyn Write>
            }
            CommandOutput::File(f) => {
                log::debug!("Writing to {}", f.display());
                Box::new(File::create(f)?)
            }
        });
        for row in rows {
            csv_writer.serialize(row)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum CommandOutput {
    Stdin,
    File(PathBuf),
}

impl FromStr for CommandOutput {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "-" => Self::Stdin,
            _ => Self::File(PathBuf::from_str(s)?),
        })
    }
}
