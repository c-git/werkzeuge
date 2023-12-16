//! Stores Command Line Interface (cli)  configuration
use clap::Parser;
use one_log::LogLevel;

#[derive(Parser, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
#[command(author, version, about)]
/// Stores the configurations acquired via the command line
pub struct Cli {
    /// Set logging level to use
    #[arg(long, short, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,

    #[arg(value_name = "Original Text")]
    /// The text to be converted to snake case
    pub org_str: String,
}
