use anyhow::Result;
use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(version = "0.1")]
#[clap(setting = AppSettings::SubcommandRequiredElseHelp)]
pub(crate) struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub(super) enum SubCommand {
    #[clap(name = "refactor_deepsize")]
    Empty(EmptyCmd),
}

#[derive(Clap, Debug)]
pub(crate) struct EmptyCmd {}

impl EmptyCmd {
    pub(crate) fn handle(&self) -> Result<()> {
        println!("Hello World");
        Ok(())
    }
}
