use clap::Parser;
use snake_case::{self, run, Cli};
use tracing::debug;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() -> anyhow::Result<()> {
    let cli: Cli = Cli::parse();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new(if cfg!(debug_assertions) {
                "info"
            } else {
                "warn"
            })
        }))
        .init();

    debug!("Cli: {cli:#?}");
    let converted_text = run(&cli);
    println!("{converted_text}");
    Ok(())
}
