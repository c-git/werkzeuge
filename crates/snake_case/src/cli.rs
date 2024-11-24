//! Stores Command Line Interface (cli)  configuration
use clap::Parser;

#[derive(Parser, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
#[command(author, version, about)]
/// Stores the configurations acquired via the command line
pub struct Cli {
    #[arg(value_name = "Original Text")]
    /// The text to be converted to snake case
    pub org_str: String,
}
