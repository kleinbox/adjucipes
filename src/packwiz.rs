use anyhow::{anyhow, Context, Result};
use camino::{Utf8PathBuf, Utf8Path};
use toml::Table;
use std::fs::read_to_string;
use std::collections::HashMap;

const HEADER: &str = "pack.toml";

pub struct Packwiz {

}

struct Mod {

}

pub fn get_instance(path: &Option<Utf8PathBuf>) -> Result<Packwiz> {
    let cur_path = match path {
        Some(path) => path.to_owned(),
        None => Utf8PathBuf::new().join("./")
    };
    let cur_path = cur_path.as_path();

    let packwiz_home = find_home(cur_path, path.is_some())
        .context("Packwiz instance has not been found")?;
    let mods = get_mods(&packwiz_home, HashMap::new())
        .context("Could not get the entries of the index TOMl file")?;


    Ok(Packwiz {

    })
}

fn find_home(path: &Utf8Path, repeat: bool) -> Result<Utf8PathBuf> {
    let mut cur_path = Utf8PathBuf::new().join(path);

    // Find packwiz instance
    while !cur_path.join(HEADER).exists() {
        if !repeat {
            return Err(anyhow!("File {} does not exist in path {}", HEADER, cur_path));
        }

        cur_path = match cur_path.parent() {
            Some(cur_path) => cur_path.to_owned(),
            None => { return Err(anyhow!("Could not find Packwiz instance")); },
        };
    }
    
    Ok(cur_path)
}

fn get_mods(packwiz_home: &Utf8PathBuf, modid_whitelist: HashMap<String, String>) -> Result<HashMap<String, Mod>> {
    let header = read_to_string(packwiz_home.join(HEADER))
        .context("pack.toml could not be read")?
        .parse::<Table>().context("pack.toml is not an valid TOML file")?;

    let mut packwiz_index = match &header["index"]["file"] {
        toml::Value::String(value) => packwiz_home.clone().join(value).to_owned(),
        _ => { return Err(anyhow!("Unexpected value for 'index.index' in pack.toml")); },
    };
    packwiz_index = packwiz_home.join(packwiz_index);

    let index = match &read_to_string(packwiz_index)
        .context("pack.toml could not be read")?
        .parse::<Table>().context("index file is not an valid TOML file")?["files"] {
            toml::Value::Array(value) => value,
            _ => &Vec::<toml::Value>::new(),
        };


    todo!()
}

//fn get_recipes(index)