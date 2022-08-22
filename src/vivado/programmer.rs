use anyhow::{anyhow, Result};
use serde::Serialize;
use std::{
    env,
    process::{Command, Stdio},
};

use crate::{manifest::Manifest, programmer::Programmer, templates::render_to_file};

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
        let bitstream_path = manifest.get_bitstream_path()?;
        let context = ProgrammingContext {
            bitstream_path,
            target_device: manifest.get_target_device()?,
        };
        render_to_file("vivado/program.tcl", context, build_tcl_path)?;
        let pwd = env::current_dir()?;
        env::set_current_dir(&build_path)?;
        match vivado_batch_run("program.tcl") {
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
    match output.status.success() {
        true => Ok(()),
        false => Err(anyhow!("Vivado command returned failure")),
    }
}

fn vivado_batch_run(source: &str) -> Result<()> {
    vivado(vec!["-mode", "batch", "-source", source])
}
