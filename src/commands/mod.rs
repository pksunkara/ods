use clap::Parser;

use crate::error::Result;

pub mod generate;
pub mod lint;

#[derive(Debug, Parser)]
pub enum Subcommands {
    #[clap(aliases = &["gen", "g"])]
    Generate(generate::Generate),
    Lint(lint::Lint),
}

impl Subcommands {
    pub(crate) fn run(&self) -> Result {
        match self {
            Self::Generate(x) => x.run(),
            Self::Lint(x) => x.run(),
        }
    }
}
