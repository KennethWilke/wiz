use anyhow::{anyhow, Result};
use serde::Serialize;
use std::{
    env, fs,
    process::{Command, Stdio},
};
use tera::Context;

use super::Builder;
use crate::{manifest::Manifest, templates::render_to_file};

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
}

impl Builder for VivadoBuilder {
    fn build(&self, manifest: Manifest) -> Result<()> {
        let build_path = self.get_build_directory(Some("build-vivado".into()))?;
        let build_tcl_path = build_path.join("build.tcl");
        let build_tcl_path = match build_tcl_path.to_str() {
            Some(path) => path,
            None => return Err(anyhow!("failed to get path to tmpdir")),
        };

        let project_name = manifest.package_name.clone();
        let target_part = manifest
            .target_part
            .as_ref()
            .expect("expected target_part")
            .to_string();
        let sources = manifest.get_source_files()?;
        let constraints = manifest.get_constraint_files()?;
        let num_jobs = num_cpus::get();

        let context = BuildContext {
            project_name,
            target_part,
            sources,
            constraints,
            num_jobs,
        };
        let context = Context::from_serialize(context)?;
        render_to_file("vivado/build.tcl", &context, build_tcl_path)?;
        let pwd = env::current_dir()?;
        env::set_current_dir(&build_path)?;
        match vivado_batch("build.tcl") {
            Ok(_) => {}
            Err(error) => {
                env::set_current_dir(pwd)?;
                return Err(error);
            }
        };
        let bitstream_path = format!("vivado/{}.runs/impl_1/top.bit", manifest.package_name);
        fs::copy(bitstream_path, manifest.get_bitstream_path()?)?;

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
