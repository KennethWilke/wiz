use std::path::PathBuf;

use anyhow::Result;
use lazy_static::lazy_static;
use serde::Serialize;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let template_path = format!(
            "{}/templates/**/*",
            std::env::var("WIZ_PATH").expect("Failed to get WIZ_PATH env var")
        );
        match Tera::new(&template_path) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}

pub fn render_template(template_name: &str, context: impl Serialize) -> Result<String> {
    let context = Context::from_serialize(context)?;
    Ok(TEMPLATES.render(template_name, &context)?)
}

pub fn render_to_file(template_name: &str, context: impl Serialize, path: PathBuf) -> Result<()> {
    let contents = render_template(template_name, context)?;
    std::fs::write(path, contents)?;
    Ok(())
}
