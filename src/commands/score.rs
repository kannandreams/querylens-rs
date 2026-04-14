use std::path::Path;

use anyhow::Result;
use clap::ValueEnum;
use crate::config::Config;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum ScoreFormat {
    Json,
    Badge,
}

pub fn run_score(path: &Path, _config: &Config, min_score: Option<u8>, detail: bool, format: ScoreFormat) -> Result<()> {
    println!("Scoring {}", path.display());
    println!("Minimum score: {:?}", min_score);
    println!("Detail mode: {}", detail);
    println!("Score format: {:?}", format);
    println!("(Score implementation placeholder)");
    Ok(())
}
