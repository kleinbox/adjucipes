use anyhow::{Context, Result};
use camino::Utf8PathBuf;

use crate::datapack;
use crate::exceptions::Exception;

pub fn exec(path: Utf8PathBuf, _destination: Utf8PathBuf) -> Result<()> {
    // Check if path is empty
    let _instance = datapack::get_instance(&path)
        .context(Exception::NotFound { expected: "instance of Abstract Datapack", location: Box::leak(path.to_string().into_boxed_str()) })?;

    todo!();
}