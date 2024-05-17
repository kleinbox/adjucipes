use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Generate Minecraft Datapacks through TOML and Lua files.")]
#[command(long_about = "Adjucipes is a CLI tool that can generate Minecraft Datapacks.\nFor those, folders defined as Abstract Datapacks are being created which define how the final Datapack will be generated.\nSupported are TOML files for static things like blacklists and scripting through Lua files, allowing more complex automation as well.")]
#[command(propagate_version = true)]
pub struct Cli {
    /// The CLI command to execute instead of the TUI
    #[command(subcommand)]
    pub command: Option<Command>
}

/// Open a TUI by default or run a command through the CLI
#[derive(Subcommand)]
pub enum Command {
    /// Create a new Abstract Datapack with a TUI
    New {
        /// The path where the command should be executed
        #[arg(short, long, default_value = "./")]
        path: Utf8PathBuf,

        /// Skip the TUI and use the default template instead
        #[arg(short, long)]
        now: bool,
    },

    /// Configuration of the Abstract Datapack
    Settings {
        /// The path where the command should be executed
        #[arg(short, long, default_value = "./")]
        path: Utf8PathBuf,

        /// The key of the option to access
        key: Option<String>,
        /// The new value for the given key. Leave blank to only read
        value: Option<String>,
    },

    /// Generate a Datapack
    Build {
        /// The path where the command should be executed
        #[arg(short, long, default_value = "./")]
        path: Utf8PathBuf,

        /// Determine the path where the Datapack should be stored.
        #[arg(default_value = "./")]
        destination: Utf8PathBuf,
    },
}

pub mod new;
pub mod settings;
pub mod build;