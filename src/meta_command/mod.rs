use crate::error::{Result, SQLRiteError};

use std::fmt;
use rustyline::Editor;
use crate::repl::{REPLHelper};

#[derive(Debug, PartialEq)]
pub enum MetaCommand {
    Exit,
    Help,
    Open(String),
    Unknown,
}

/// Trait responsible for translating type into a formated text.
impl fmt::Display for MetaCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MetaCommand::Exit => f.write_str(".шухер"),
            MetaCommand::Help => f.write_str(".передачка"),
            MetaCommand::Open(_) => f.write_str(".захомячить"),
            MetaCommand::Unknown => f.write_str("Ты че охуел???"),
        }
    }
}

impl MetaCommand {
    pub fn new(command: String) -> MetaCommand {
        let args: Vec<&str> = command.split_whitespace().collect();
        let cmd = args[0].to_owned();
        match cmd.as_ref() {
            ".шухер" => MetaCommand::Exit,
            ".передачка" => MetaCommand::Help,
            ".захомячить" => MetaCommand::Open(command),
            _ => MetaCommand::Unknown,
        }
    }
}

pub fn handle_meta_command(command: MetaCommand, repl: &mut Editor<REPLHelper>) -> Result<String> {
    match command {
        MetaCommand::Exit => {
            repl.append_history("history").unwrap();
            std::process::exit(0)
        },
        MetaCommand::Help => Ok(format!(
            "{}{}{}",
            "Special commands:\n",
            ".передачка        - Display this message\n",
            ".шухер            - Quits this application"
        )),
        MetaCommand::Open(args) => Ok(format!("To be implemented: {}", args)),
        MetaCommand::Unknown => Err(SQLRiteError::UnknownCommand(format!(
            "Unknown command or invalid arguments. Enter '.help'"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repl::{get_config, REPLHelper};

    #[test]
    fn get_meta_command_exit_test() {
        // Starting Rustyline with a default configuration
        let config = get_config();

        // Getting a new Rustyline Helper
        let helper = REPLHelper::default();

        // Initiatlizing Rustyline Editor with set config and setting helper
        let mut repl = Editor::with_config(config);
        repl.set_helper(Some(helper));

        let inputed_command = MetaCommand::Help;

        let result = handle_meta_command(inputed_command, &mut repl);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn get_meta_command_open_test() {
        // Starting Rustyline with a default configuration
        let config = get_config();

        // Getting a new Rustyline Helper
        let helper = REPLHelper::default();

        // Initiatlizing Rustyline Editor with set config and setting helper
        let mut repl = Editor::with_config(config);
        repl.set_helper(Some(helper));

        let inputed_command = MetaCommand::Open(".open database.db".to_string());

        let result = handle_meta_command(inputed_command, &mut repl);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn get_meta_command_unknown_command_test() {
        // Starting Rustyline with a default configuration
        let config = get_config();

        // Getting a new Rustyline Helper
        let helper = REPLHelper::default();

        // Initiatlizing Rustyline Editor with set config and setting helper
        let mut repl = Editor::with_config(config);
        repl.set_helper(Some(helper));

        let inputed_command = MetaCommand::Unknown;

        let result = handle_meta_command(inputed_command, &mut repl);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn meta_command_display_trait_test() {
        let exit = MetaCommand::Exit;
        let help = MetaCommand::Help;
        let open = MetaCommand::Open(".open database.db".to_string());
        let unknown = MetaCommand::Unknown;

        assert_eq!(format!("{}", exit), ".exit");
        assert_eq!(format!("{}", help), ".help");
        assert_eq!(format!("{}", open), ".open");
        assert_eq!(format!("{}", unknown), "Unknown command");
    }
}
