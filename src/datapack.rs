use std::io::Cursor;
use anyhow::{anyhow, Context, Result};
use camino::Utf8Path;
use zip::ZipArchive;

use crate::exceptions::Exception;

const HEADER: &str = "datapack.toml";

pub struct Datapack {

}

pub fn get_instance(path: &Utf8Path) -> Result<Datapack> {
    if !path.join(HEADER).exists() {
        return Err(anyhow!("File {} does not exist in path {}", HEADER, path));
    }

    Ok(Datapack {

    })
}

pub fn generate_abstract_template(path: &Utf8Path) -> Result<()> {
    let zip_template = include_bytes!("template.zip");
    let mut archive = ZipArchive::new(Cursor::new(zip_template))
        .context(Exception::Panic { reason: "Could not gather the data for the template" })?;

    if let Err(error) = archive.extract(path) {
        return Err(anyhow!(Exception::ZipError(format!("the path {}", path).to_string(), error)))
    }

    Ok(())
}