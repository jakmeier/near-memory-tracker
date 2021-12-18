mod opts;

use crate::opts::{Opts, SubCommand};
use clap::Parser;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_writer(std::io::stderr).finish().init();
    info!("init");
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Empty(empty_cmd) => empty_cmd.handle()?,
    };
    Ok(())
}
