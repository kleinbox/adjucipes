use anyhow::Error;
use std::process::exit;
use nu_ansi_term::{Style, Color};

pub fn throw(error: &Error, exitcode: exitcode::ExitCode) -> ! {
    println!("{}", Style::default().fg(Color::Red).paint("Program has been aborted due to an exception!"));

    println!("\n{}\t{}",
        Style::default().underline().paint("Reason:"),
        Style::default().bold().paint(error.to_string()),
    );

    if error.chain().count() > 1 {
        println!("\n{}", Style::default().underline().paint("Stacktrace:"));
        for cause in error.chain().skip(1) {
            println!("\t{}", Style::default().paint(cause.to_string()));
        }
    }

    exit(exitcode)
}

#[derive(thiserror::Error, Debug)]
pub enum Exception {
    #[error("Failed to get {expected} from {location}")]
    NotFound {
        expected: &'static str,
        location:  &'static str,
    },
    #[error("{thing} already exists")]
    AlreadyExists {
        thing: &'static str,
    },
    #[error("Something impossible just happened: {reason}")]
    Panic {
        reason: &'static str,
    },
    #[error("An error has been encurranged within {0}: {1}")]
    ZipError(String, zip::result::ZipError),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

impl Exception {
    pub fn exitcode(&self) -> exitcode::ExitCode {
        match self {
            Exception::NotFound { expected: _, location: _ } => exitcode::DATAERR,
            Exception::AlreadyExists { thing: _ } => exitcode::DATAERR,
            Exception::Panic { reason: _ } => exitcode::SOFTWARE,
            Exception::ZipError(_, _) => exitcode::IOERR,
            Exception::InvalidConfig(_) => exitcode::DATAERR,
        }
    }
}