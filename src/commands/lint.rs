use std::path::Path;

use anyhow::Result;
use crate::config::Config;

pub fn run_lint(path: &Path, config: &Config, rules: &[String], severity: Option<&str>) -> Result<()> {
    println!("Running lint on {}", path.display());
    println!("Using dialect: {}", config.dialect);
    println!("Enabled rules: {}", config.rules.len());
    if !rules.is_empty() {
        println!("Selected rule filters: {:?}", rules);
    }
    if let Some(severity) = severity {
        println!("Severity filter: {}", severity);
    }
    println!("(Lint implementation placeholder)");
    Ok(())
}
