use anyhow::Result;
use std::path::Path;

use crate::{helpers::get_absolute_path, templates::render_to_file};

use super::BuildContext;

pub fn generate_pins(build_path: &Path, context: &mut BuildContext) -> Result<()> {
    let xdc_path = build_path.join("pins.xdc");

    println!("Generating pins");
    render_to_file("vivado/pins.xdc", &context, xdc_path.clone())?;
    context.add_contraint(get_absolute_path(
        xdc_path.to_str().expect("path expected to be string"),
    ));
    Ok(())
}
