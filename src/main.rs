mod builder;
mod helpers;
mod manifest;
mod programmer;
mod templates;
mod vivado;

use anyhow::Result;
use builder::Builder;
use clap::{Parser, Subcommand};

use crate::{manifest::Manifest, programmer::Programmer};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(short, long, value_parser, default_value = "wiz.json")]
    manifest: String,
}

#[derive(Subcommand, Debug)]
#[clap()]
enum Command {
    Build,
    Program,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let manifest = Manifest::load(&args.manifest)?;
    //println!("{:#?}", manifest);

    use Command::*;
    match args.command {
        Build => build(manifest),
        Program => program(manifest),
    }
}

fn build(manifest: Manifest) -> Result<()> {
    let builder_name = manifest.get_builder_name()?;
    println!(
        "Building '{}' with {}",
        manifest.get_package_name(),
        builder_name
    );
    let builder = manifest.get_builder()?;
    builder.build(manifest)
}

fn program(manifest: Manifest) -> Result<()> {
    let programmer_name = manifest.get_programmer_name()?;
    println!(
        "Programming '{}' with {}",
        manifest.get_package_name(),
        programmer_name
    );
    let programmer = manifest.get_programmer()?;
    programmer.program(manifest)
}
