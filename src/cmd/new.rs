use camino::Utf8PathBuf;
use anyhow::{Result, anyhow};

use crate::datapack;
use crate::exceptions::Exception;

pub fn exec(path: Utf8PathBuf, now: bool) -> Result<()> {
    if datapack::get_instance(&path).is_ok() {
        return Err(anyhow!(Exception::AlreadyExists { thing: "Abstract Datapack" }))
    } else if path.read_dir()?.next().is_some() {
        return Err(anyhow!(Exception::InvalidConfig("Folder must be empty for a new Abstract Datapack".to_string())))
    }

    if now {
        datapack::generate_abstract_template(&path)?;
        Ok(())
    } else {
        todo!(); // Add TUI here
    }
}