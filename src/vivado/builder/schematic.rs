use std::{env, path::PathBuf};

use crate::{
    manifest::Manifest,
    templates::render_to_file,
    vivado::{builder::BuildContext, vivado_batch},
};
use anyhow::Result;

pub fn show_schematic(build_path: PathBuf, manifest: Manifest) -> Result<()> {
    println!("Generating schematic script");
    let context = BuildContext::try_from(&manifest)?;
    let build_tcl_path = build_path.join("schematic.tcl");
    render_to_file("vivado/schematic.tcl", &context, build_tcl_path)?;

    let pwd = env::current_dir()?;
    env::set_current_dir(&build_path)?;
    match vivado_batch("schematic.tcl") {
        Ok(_) => {}
        Err(error) => {
            env::set_current_dir(pwd)?;
            return Err(error);
        }
    };

    env::set_current_dir(pwd)?;
    Ok(())
}
