use anyhow::Result;
use std::{env, fs, path::PathBuf};

use crate::{manifest::Manifest, templates::render_to_file, vivado::vivado_batch};

use super::{generate_pins::generate_pins, BuildContext};

pub fn build(build_path: PathBuf, manifest: Manifest) -> Result<()> {
    let build_tcl_path = build_path.join("build.tcl");
    let project_name = manifest.get_package_name();
    let mut context = BuildContext::try_from(&manifest)?;

    if manifest.maps_pins() {
        generate_pins(&build_path, &mut context)?;
    }

    println!("Generating build script");
    generate_build_script(build_tcl_path, &context)?;

    let pwd = env::current_dir()?;
    env::set_current_dir(&build_path)?;
    match vivado_batch("build.tcl") {
        Ok(_) => {}
        Err(error) => {
            env::set_current_dir(pwd)?;
            return Err(error);
        }
    };
    let bitstream_path = format!("vivado/{}.runs/impl_1/top.bit", project_name);
    fs::copy(bitstream_path, manifest.get_bitstream_path()?)?;

    env::set_current_dir(pwd)?;
    Ok(())
}

fn generate_build_script(script_path: PathBuf, context: &BuildContext) -> Result<()> {
    render_to_file("vivado/build.tcl", &context, script_path)
}
