use std::env::Args;

use eyre::{Report, Result};

/// Parses given arguments and checks for arguments conditions
pub struct ArgParser {
    pub help: bool,
    pub add: bool,
    pub list: bool,
    pub session: String,
}

impl ArgParser {
    /// Parses arguments and creates new [`ArgParser`]
    /// * `args` - Args to be parsed
    pub fn parse(args: Args) -> Result<Self> {
        let mut parser = Self::default();

        if args.len() > 2 {
            return Err(Report::msg("Invalid number of arguments"));
        }

        for arg in args.skip(1) {
            match arg.as_str() {
                "-h" | "--help" => parser.help = true,
                "-a" | "--add" => parser.add = true,
                "-l" | "--list" => parser.list = true,
                _ => {
                    if !parser.session.is_empty() {
                        return Err(Report::msg("Multiple sessions provided"));
                    }
                    parser.session = arg;
                }
            }
        }

        Ok(parser)
    }
}

impl Default for ArgParser {
    fn default() -> Self {
        Self {
            help: false,
            add: false,
            list: false,
            session: "".to_owned(),
        }
    }
}
