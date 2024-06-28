use anyhow::{Context, Result};
use camino::Utf8PathBuf;

use crate::datapack;
use crate::exceptions::Exception;
use crate::packwiz;

pub fn exec(
    path: Utf8PathBuf,
    destination: Utf8PathBuf,
    packwiz_path: Option<Utf8PathBuf>,
) -> Result<()> {
    // Check if path is empty
    //let _instance = datapack::get_instance(&path)
    //    .context(Exception::NotFound { expected: "instance of Abstract Datapack", location: Box::leak(path.to_string().into_boxed_str()) })?;

    let packwiz_instance = packwiz::get_instance(&packwiz_path).context(Exception::NotFound {
        expected: "instance of Packwiz",
        location: "given Packwiz path or it's parent folders",
    })?;

    todo!();
}
