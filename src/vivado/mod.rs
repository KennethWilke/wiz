use anyhow::Result;
use std::process::{Command, Stdio};

pub mod builder;
pub mod programmer;

pub fn vivado(args: Vec<&str>) -> Result<()> {
    let output = Command::new("vivado")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    println!("{}", output.status);
    Ok(())
}

pub fn vivado_batch(source: &str) -> Result<()> {
    vivado(vec!["-mode", "batch", "-source", source])
}
