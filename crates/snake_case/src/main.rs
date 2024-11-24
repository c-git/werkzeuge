use clap::Parser;
use log::debug;
use snake_case::{self, run, Cli};
fn main() -> anyhow::Result<()> {
    let cli: Cli = Cli::parse();
    // TODO 1: Add tracing to replace log
    debug!("Cli: {cli:#?}");
    let converted_text = run(&cli);
    println!("{converted_text}");
    Ok(())
}
