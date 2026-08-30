use anyhow::Result;
use clap::Parser;
use serde_yaml::Value;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// YAML file to dump
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let content = fs::read_to_string(&args.file)?;
    let yaml: Value = serde_yaml::from_str(&content)?;
    println!("{}", serde_yaml::to_string(&yaml)?);
    Ok(())
}
