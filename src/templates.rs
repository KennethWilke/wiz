use anyhow::Result;
use lazy_static::lazy_static;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}

pub fn render_template(template_name: &str, context: &Context) -> Result<String> {
    Ok(TEMPLATES.render(template_name, context)?)
}

pub fn render_to_file(template_name: &str, context: &Context, path: &str) -> Result<()> {
    let contents = render_template(template_name, context)?;
    std::fs::write(path, contents)?;
    Ok(())
}
