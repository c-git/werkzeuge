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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Simple Case", "simple_case")]
    #[case("With !@#$%^&*()_+/\\ Special *Chars", "with_special_chars")]
    #[case(
        "Task 2: Unanimously Legendary IDentifier (ULID) ",
        "task_2_unanimously_legendary_i_dentifier_ulid"
    )]
    #[case(
        "Task 2: Unanimously Legendary IDentifier [ULID] ",
        "task_2_unanimously_legendary_i_dentifier_ulid"
    )]
    fn conversion(#[case] input: &str, #[case] expected: &str) {
        let actual = convert_to_snake_case(input);
        assert_eq!(actual, expected);
    }
}
