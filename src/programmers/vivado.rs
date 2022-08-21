use anyhow::{anyhow, Result};
use serde::Serialize;
use std::process::{Command, Stdio};
use tera::Context;

use super::Programmer;
use crate::{manifest::Manifest, templates::render_to_file};

pub struct VivadoProgrammer {}

impl VivadoProgrammer {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize, Debug)]
pub struct ProgrammingContext {
    project_name: String,
    target_device: String,
}

impl Programmer for VivadoProgrammer {
    fn program(&self, manifest: Manifest) -> Result<()> {
        let build_path = self.get_build_directory(Some("build-vivado".into()))?;
        let build_tcl_path = build_path.join("program.tcl");
        let build_tcl_path = match build_tcl_path.to_str() {
            Some(path) => path,
            None => return Err(anyhow!("failed to get path to tmpdir")),
        };
        let context = ProgrammingContext {
            project_name: manifest.package_name.clone(),
            target_device: manifest
                .target_device
                .expect("target_device expected"),
        };
        let context = Context::from_serialize(context)?;
        render_to_file("vivado/program.tcl", &context, build_tcl_path)?;
        vivado_batch(build_tcl_path)
    }
}

fn vivado(args: Vec<&str>) -> Result<()> {
    let output = Command::new("vivado")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    println!("{}", output.status);
    Ok(())
}

fn vivado_batch(source: &str) -> Result<()> {
    vivado(vec!["-mode", "batch", "-source", source])
}
