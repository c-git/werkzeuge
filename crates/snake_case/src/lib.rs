#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
// For documenting optional features. See more at <https://c-git.github.io/rust/documentation/>
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
//! Converts text into snake case

use convert_case::{Case, Casing};

mod cli;

pub use cli::Cli;
use regex::Regex;

/// Runs the body of the logic
pub fn run(cli: &Cli) -> String {
    convert_to_snake_case(&cli.org_str)
}

fn convert_to_snake_case(org_str: &str) -> String {
    let re = Regex::new("[^0-9a-zA-Z_ ]").unwrap();
    re.replace_all(org_str, "").to_case(Case::Snake)
}
