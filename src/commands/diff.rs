use std::path::Path;

use anyhow::Result;
use clap::ValueEnum;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum DiffMode {
    Text,
    Structure,
    Both,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum DiffFormat {
    Unified,
    Json,
}

pub fn run_diff(
    before: &Path,
    after: &Path,
    mode: DiffMode,
    format: DiffFormat,
    show_unchanged: bool,
) -> Result<()> {
    println!("Comparing {} -> {}", before.display(), after.display());
    println!("Mode: {:?}", mode);
    println!("Format: {:?}", format);
    println!("Show unchanged: {}", show_unchanged);
    println!("(Diff implementation placeholder)");
    Ok(())
}
