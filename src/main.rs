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

    #[clap(short, long, value_parser, default_value = "wiz.toml")]
    manifest: String,
}

#[derive(Subcommand, Debug)]
#[clap()]
enum Command {
    Build { subcommand: Option<String> },
    Program,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let manifest = match Manifest::load(&args.manifest) {
        Ok(manifest) => manifest,
        Err(error) => {
            println!("Failed to load manifest file \"{}\"", &args.manifest);
            return Err(error)
        }
    };
    println!("{}", toml::to_string_pretty(&manifest)?);

    use Command::*;
    match args.command {
        Build { subcommand } => match subcommand {
            Some(subcommand) => build_subcommand(manifest, subcommand),
            None => build(manifest),
        },
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

fn build_subcommand(manifest: Manifest, subcommand: String) -> Result<()> {
    let builder_name = manifest.get_builder_name()?;
    println!(
        "Building '{}' with {} {}",
        manifest.get_package_name(),
        builder_name,
        subcommand
    );
    let builder = manifest.get_builder()?;
    builder.subcommand(manifest, subcommand, vec![])
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
