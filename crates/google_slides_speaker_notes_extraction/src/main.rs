use std::{fmt::Write, fs, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use tracing::trace;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "
Program to convert a \"Google Slide\" file that has been downloaded as plain text into only the speaker notes and page numbers

Assumes:
- each slide has a page number that comes last on the slide
- Each page has a string on a line on it's own that can be used to make the end of the speaker notes for that page
- Pages may contain numbers but they must not contain the page number
"
)]
// #[command(long)]
struct Cli {
    #[arg()]
    /// The file to load and modify
    filename: PathBuf,

    #[arg(long, short, default_value = "PTO")]
    new_page_marker: String,
}

#[derive(Debug, Default)]
enum State {
    #[default]
    /// Same as slide but will accept any page number as next page number
    Start,

    /// On slide text to be discarded
    Slide { page_number: usize },

    /// Speaker text that should not be removed
    SpeakerNotes { page_number: usize },
}
impl State {
    fn current_page_num(&self) -> Option<&usize> {
        match self {
            State::Start => None,
            State::Slide { page_number } | State::SpeakerNotes { page_number } => Some(page_number),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = &Cli::parse();
    tracing_subscriber::fmt::init();
    let file_contents = fs::read_to_string(&cli.filename)
        .with_context(|| format!("failed to read file: {:?}", cli.filename))?;
    let mut output = String::new();
    let mut state = State::default();
    for (i, line) in file_contents.lines().enumerate() {
        trace!(i);
        trace!(?state);
        match (&state, get_page_num(state.current_page_num(), line)) {
            (State::Start, None) | (State::Slide { .. }, None) => {} // Do nothing ignore line
            (State::Start, Some(page_number)) => {
                // Start of first page (after ignoring slide text)
                writeln!(&mut output, "{line}").expect("failed to write into output buffer");
                state = State::SpeakerNotes { page_number };
            }
            (State::Slide { .. }, Some(page_number)) => {
                // Start of a new page after ignoring slide text
                writeln!(&mut output, "{line}").expect("failed to write into output buffer");
                state = State::SpeakerNotes { page_number };
            }
            (State::SpeakerNotes { page_number }, None) => {
                if line == cli.new_page_marker {
                    // New page
                    state = State::Slide {
                        page_number: *page_number,
                    };
                } else {
                    // Speaker notes that we want to keep
                    writeln!(&mut output, "{line}").expect("failed to write into output buffer");
                }
            }
            (State::SpeakerNotes { .. }, Some(page_number)) => {
                // Maybe this happens if a page has NO TEXT
                writeln!(&mut output, "{line}").expect("failed to write into output buffer");
                state = State::SpeakerNotes { page_number };
            }
        }
    }
    fs::write(&cli.filename, output)
        .with_context(|| format!("failed to write file: {:?}", cli.filename))?;
    println!("Completed");
    Ok(())
}

/// Has a number only on the line and it's the correct page number
/// Needs to ignore other numbers
fn get_page_num(current_page_number: Option<&usize>, line: &str) -> Option<usize> {
    // TODO 4: Use a lazy cell to avoid recompiling
    let re = regex::Regex::new(r"^(\d+)$").unwrap();
    let caps = re.captures(line)?;
    let result = caps[1].parse().expect("should be valid based on regex");
    if let Some(current_page_number) = current_page_number {
        if current_page_number + 1 != result {
            trace!("Number found: {result} but not page number");
            return None;
        }
    }
    trace!("Page number found: {result}");
    Some(result)
}
