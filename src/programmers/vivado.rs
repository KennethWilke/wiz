use anyhow::{anyhow, Result};
use serde::Serialize;
use std::{
    env,
    process::{Command, Stdio},
};
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
    bitstream_path: String,
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
        let bitstream_path = manifest.get_bitstream_path()?;
        let context = ProgrammingContext {
            bitstream_path,
            target_device: manifest.target_device.expect("target_device expected"),
        };
        let context = Context::from_serialize(context)?;
        render_to_file("vivado/program.tcl", &context, build_tcl_path)?;
        let pwd = env::current_dir()?;
        env::set_current_dir(&build_path)?;
        match vivado_batch("program.tcl") {
            Ok(_) => {
                env::set_current_dir(pwd)?;
                Ok(())
            }
            Err(error) => {
                env::set_current_dir(pwd)?;
                Err(error)
            }
        }
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
