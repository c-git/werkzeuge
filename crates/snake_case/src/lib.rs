#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
// For documenting optional features. See more at <https://c-git.github.io/rust/documentation/>
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
//! Converts text into snake case

use convert_case::{Case, Casing};

mod cli;

pub use cli::Cli;

/// Runs the body of the logic
pub fn run(cli: &Cli) -> String {
    convert_to_snake_case(&cli.org_str)
}

fn convert_to_snake_case(org_str: &str) -> String {
    org_str.to_case(Case::Snake)
}
