use std::{fs::File, io::Read};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{
    builders::{vivado::VivadoBuilder, Builder},
    helpers,
    programmers::{vivado::VivadoProgrammer, Programmer},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub package_name: String,
    pub builder: Option<String>,
    pub programmer: Option<String>,
    pub target_part: Option<String>,
    pub target_device: Option<String>,
    source_files: Option<Vec<String>>,
    constraint_files: Option<Vec<String>>,
    bitstream_path: Option<String>,
}

impl Manifest {
    pub fn load(path: impl ToString) -> Result<Self> {
        let mut file = File::open(path.to_string())?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(serde_json::from_str(&buffer)?)
    }

    pub fn get_builder(&self) -> Result<impl Builder> {
        let name = match &self.builder {
            Some(builder) => builder,
            None => return Err(anyhow!("No builder in manifest")),
        };
        Ok(match name.as_str() {
            "vivado" => VivadoBuilder::new(),
            unhandled => return Err(anyhow!(format!("Unhandled builder: {}", unhandled))),
        })
    }

    pub fn get_programmer(&self) -> Result<impl Programmer> {
        let name = match &self.programmer {
            Some(programmer) => programmer,
            None => return Err(anyhow!("No programmer in manifest")),
        };
        Ok(match name.as_str() {
            "vivado" => VivadoProgrammer::new(),
            unhandled => return Err(anyhow!(format!("Unhandled programmer: {}", unhandled))),
        })
    }

    pub fn get_source_files(&self) -> Result<Vec<String>> {
        match &self.source_files {
            Some(sources) => {
                let mut files = vec![];
                for file in sources {
                    files.push(helpers::get_absolute_path(file))
                }
                Ok(files)
            }
            None => Ok(vec![]),
        }
    }

    pub fn get_bitstream_path(&self) -> Result<String> {
        match &self.bitstream_path {
            Some(path) => Ok(path.to_string()),
            None => Err(anyhow!("No bitstream path defined")),
        }
    }

    pub fn get_constraint_files(&self) -> Result<Vec<String>> {
        match &self.constraint_files {
            Some(constraints) => {
                let mut files = vec![];
                for file in constraints {
                    files.push(helpers::get_absolute_path(file))
                }
                Ok(files)
            }
            None => Ok(vec![]),
        }
    }
}
