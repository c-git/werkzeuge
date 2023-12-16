use anyhow::Context;
use clap::Parser;
use log::debug;
use one_log::init_logging;
use snake_case::{self, run, Cli};
fn main() -> anyhow::Result<()> {
    let cli: Cli = Cli::parse();
    init_logging(cli.log_level.into())?;
    debug!("Cli: {cli:#?}");
    let converted_text = run(&cli).context("Failed to convert input")?;
    println!("{converted_text}");
    Ok(())
}
