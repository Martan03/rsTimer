use termint::{
    enums::Color,
    help,
    widgets::{Grad, StrSpanExtension},
};

use crate::error::Error;

#[derive(Debug)]
pub enum Action {
    Add,
    Help,
    List,
}

/// Parses given arguments and checks for arguments conditions
#[derive(Debug, Default)]
pub struct Args {
    pub action: Option<Action>,
    pub session: Option<String>,
}

impl Args {
    /// Parses arguments
    pub fn parse(args: std::env::Args) -> Result<Args, Error> {
        let mut parsed = Self::default();

        if args.len() > 2 {
            return Err(Error::Msg("Invalid number of arguments".to_string()));
        }

        let mut args_iter = args.into_iter();
        args_iter.next();
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-h" | "--help" => parsed.set_action(Action::Help)?,
                "-a" | "--add" => parsed.set_action(Action::Add)?,
                "-l" | "--list" => parsed.set_action(Action::List)?,
                name => parsed.set_session(name)?,
            }
        }

        Ok(parsed)
    }

    /// Displays help
    pub fn help() {
        println!(
            "Welcome to help for {} by {}\n",
            "rsTimer".fg(Color::Green),
            Grad::new("Martan03", (0, 220, 255), (175, 80, 255))
        );
        help!(
            "Usage":
            "rstimer" => "Opens session picker to choose which one to open\n"
            "rstimer" ["session name"] => "Opens timer with given session\n"
            "rstimer" ["options"] => "Behaves according to options\n"
            "Options":
            "-a  --add" => "Opens dialog to add new session\n"
            "-l  --list" => "Lists all sessions\n"
            "-h  --help" => "Prints this help"
        );
    }

    /// Sets action to given value when is not set already
    fn set_action(&mut self, action: Action) -> Result<(), Error> {
        if self.action.is_some() {
            Err(Error::Msg("multiple actions provided".to_string()))
        } else {
            self.action = Some(action);
            Ok(())
        }
    }

    /// Sets session name to given value when is not set already
    fn set_session(&mut self, name: &str) -> Result<(), Error> {
        if self.session.is_some() {
            Err(Error::Msg("multiple sessions provided".to_string()))
        } else {
            self.session = Some(name.to_string());
            Ok(())
        }
    }
}
