use anyhow::Error;
use clap::Parser;
use exceptions::Exception;
use std::process::exit;

pub mod cmd;
pub mod datapack;
pub mod exceptions;
pub mod packwiz;

fn main() {
    let cli = match cmd::Cli::try_parse() {
        Ok(result) => result,
        Err(error) => {
            // is this an error?
            if !error.use_stderr() {
                // Oh nvm just printing some informations
                if let Err(another_error) = error.print() {
                    // Wait something else went wrong wtf??
                    exceptions::throw(&Error::from(another_error), exitcode::SOFTWARE)
                }
                exit(exitcode::OK)
            }
            // Ah nope, just our expected syntax error
            exceptions::throw(&Error::from(error), exitcode::USAGE)
        }
    };

    if let Some(command) = cli.command {
        // CLI
        let result = match command {
            cmd::Command::New { path, now } => cmd::new::exec(path, now),

            cmd::Command::Settings { path, key, value } => cmd::settings::exec(path, key, value),

            cmd::Command::Build {
                path,
                destination,
                packwiz,
            } => cmd::build::exec(path, destination, packwiz),
        };

        if let Err(error) = result {
            if let Some(exception) = error.downcast_ref::<Exception>() {
                exceptions::throw(&error, exception.exitcode());
            } else {
                exit(exitcode::DATAERR);
            }
        }
    } else {
        // TODO: TUI
    }

    exit(exitcode::OK);
}
