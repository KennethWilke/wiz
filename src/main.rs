mod builders;
mod manifest;
mod programmers;
mod simulators;
mod templates;

use anyhow::Result;
use builders::Builder;
use clap::{Parser, Subcommand};

use crate::{manifest::Manifest, programmers::Programmer};

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
    Simulate,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let manifest = Manifest::load(&args.manifest)?;
    //println!("{:#?}", manifest);

    use Command::*;
    match args.command {
        Build => build(manifest),
        Program => program(manifest),
        Simulate => simulate(manifest),
    }
}

fn build(manifest: Manifest) -> Result<()> {
    let builder_name = manifest.builder.as_ref().expect("expected builder");
    println!("Building '{}' with {}", manifest.package_name, builder_name);
    let builder = manifest.get_builder()?;
    builder.build(manifest)
}

fn program(manifest: Manifest) -> Result<()> {
    let programmer_name = manifest.programmer.as_ref().expect("expected programmer");
    println!(
        "Programming '{}' with {}",
        manifest.package_name, programmer_name
    );
    let programmer = manifest.get_programmer()?;
    programmer.program(manifest)
}

fn simulate(_manifest: Manifest) -> Result<()> {
    todo!("simmmy")
}
