#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
// For documenting optional features. See more at <https://c-git.github.io/rust/documentation/>
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
//! Converts text into snake case

mod cli;

pub use cli::Cli;

/// Runs the body of the logic
pub fn run(cli: &Cli) -> anyhow::Result<String> {
    // TODO: Conversion
    Ok(cli.org_str.clone())
}
