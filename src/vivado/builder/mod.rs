use std::collections::HashMap;

use anyhow::{anyhow, Result};
use serde::Serialize;

use crate::{builder::Builder, manifest::Manifest};

use self::schematic::show_schematic;

mod build;
mod generate_pins;
mod schematic;

pub struct VivadoBuilder {}

impl VivadoBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize, Debug)]
pub struct BuildContext {
    project_name: String,
    target_part: String,
    sources: Vec<String>,
    constraints: Vec<String>,
    num_jobs: usize,
    pins: HashMap<String, String>,
}

impl BuildContext {
    pub fn add_contraint(&mut self, value: String) {
        self.sources.push(value);
    }
}

impl TryFrom<&Manifest> for BuildContext {
    type Error = anyhow::Error;

    fn try_from(manifest: &Manifest) -> Result<Self> {
        let project_name = manifest.get_package_name();
        let target_part = manifest.get_target_part()?;
        let sources = manifest.get_source_files()?;
        let constraints = manifest.get_constraint_files()?;
        let num_jobs = num_cpus::get();
        let pins = manifest.get_pins();

        let context = BuildContext {
            project_name,
            target_part,
            sources,
            constraints,
            num_jobs,
            pins,
        };
        Ok(context)
    }
}

impl Builder for VivadoBuilder {
    fn build(&self, manifest: Manifest) -> Result<()> {
        let build_path = self.get_build_directory(Some("build-vivado".into()))?;
        build::build(build_path, manifest)
    }

    fn subcommand(&self, manifest: Manifest, command: String, _args: Vec<&[u8]>) -> Result<()> {
        let build_path = self.get_build_directory(Some("build-vivado".into()))?;
        match command.as_str() {
            "schematic" => show_schematic(build_path, manifest),
            other => Err(anyhow!(format!(
                "Vivado builder has no \"{}\" subcommand",
                other
            ))),
        }
    }
}
