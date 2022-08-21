pub mod vivado;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};

use crate::manifest::Manifest;

pub trait Builder {
    fn build(&self, manifest: Manifest) -> Result<()>;

    fn get_build_directory(&self, path: Option<String>) -> Result<PathBuf> {
        let build_path = match path {
            Some(path) => path,
            None => "build".to_string(),
        };
        if !Path::new(&build_path).is_dir() {
            fs::create_dir_all(&build_path)?;
            Ok(build_path.into())
        } else {
            Ok(build_path.into())
        }
    }

    fn subcommand(&self, command: String, _args: Vec<&[u8]>) -> Result<()> {
        Err(anyhow!(format!("Builder subcommand {} not found", command)))
    }
}
