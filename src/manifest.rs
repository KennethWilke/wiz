use std::{collections::HashMap, fs::File, io::Read};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{
    builder::Builder, helpers, programmer::Programmer, vivado::builder::VivadoBuilder,
    vivado::programmer::VivadoProgrammer,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    package_name: String,
    builder: Option<String>,
    programmer: Option<String>,
    target_part: Option<String>,
    target_device: Option<String>,
    source_files: Option<Vec<String>>,
    constraint_files: Option<Vec<String>>,
    bitstream_path: Option<String>,
    pins: Option<HashMap<String, String>>,
}

impl Manifest {
    pub fn load(path: impl ToString) -> Result<Self> {
        let mut file = File::open(path.to_string())?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(serde_json::from_str(&buffer)?)
    }

    pub fn get_package_name(&self) -> String {
        self.package_name.clone()
    }

    pub fn get_builder_name(&self) -> Result<String> {
        match &self.builder {
            Some(builder) => Ok(builder.to_string()),
            None => Err(anyhow!("No target builder")),
        }
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

    pub fn get_programmer_name(&self) -> Result<String> {
        match &self.programmer {
            Some(programmer) => Ok(programmer.to_string()),
            None => Err(anyhow!("No target programmer")),
        }
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

    pub fn get_target_device(&self) -> Result<String> {
        match &self.target_device {
            Some(device) => Ok(device.to_string()),
            None => Err(anyhow!("No target device")),
        }
    }

    pub fn get_target_part(&self) -> Result<String> {
        match &self.target_part {
            Some(part) => Ok(part.to_string()),
            None => Err(anyhow!("No target part")),
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

    pub fn maps_pins(&self) -> bool {
        self.pins.is_some()
    }

    pub fn get_pins(&self) -> HashMap<String, String> {
        match &self.pins {
            Some(pins) => pins.clone(),
            None => HashMap::new(),
        }
    }
}
