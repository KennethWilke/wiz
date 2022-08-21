use std::{fs::File, io::Read};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{
    builders::{vivado::VivadoBuilder, Builder},
    programmers::{vivado::VivadoProgrammer, Programmer},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub package_name: String,
    pub builder: Option<String>,
    pub programmer: Option<String>,
    pub target_part: Option<String>,
    pub target_device: Option<String>,
    pub source_files: Option<Vec<String>>,
    pub constraint_files: Option<Vec<String>>,
    pub bitstream_path: Option<String>,
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
}
