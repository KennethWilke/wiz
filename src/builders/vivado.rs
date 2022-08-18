use std::{process::{Command, Stdio}, fs, env};
use anyhow::{Result, anyhow};
use serde::Serialize;
use tera::Context;

use crate::{manifest::Manifest, templates::render_to_file};
use super::Builder;

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
    num_jobs: usize
}

impl Builder for VivadoBuilder {
    fn build(&self, manifest: Manifest) -> Result<()> {
        let build_path = self.get_build_directory(Some("build-vivado".into()))?;
        let build_tcl_path = build_path.join("build.tcl");
        let build_tcl_path = match build_tcl_path.to_str() {
            Some(path) => path,
            None => return Err(anyhow!("failed to get path to tmpdir"))
        };
        let context = BuildContext{
            project_name: manifest.package_name.clone(),
            target_part: manifest.target_part.expect("expected target_part"),
            sources: manifest.source_files.expect("expected source_files"),
            constraints: manifest.constraint_files.expect("expected constraint_files"),
            num_jobs: num_cpus::get()
        };
        let context = Context::from_serialize(context)?;
        render_to_file("vivado/build.tcl", &context, build_tcl_path)?;
        let pwd = env::current_dir()?;
        env::set_current_dir(&build_path)?;
        match vivado_batch("build.tcl") {
            Ok(_) => {},
            Err(error) => {
                env::set_current_dir(pwd)?;
                return Err(error)
            }
        };
        let bitstream_path = format!("vivado/{}.runs/impl_1/top.bit", manifest.package_name);
        fs::copy(bitstream_path, manifest.bitstream_path.expect("expected bitstream_path"))?;

        env::set_current_dir(pwd)?;
        Ok(())
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
