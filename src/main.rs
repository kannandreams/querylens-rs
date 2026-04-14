use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod commands;
mod config;

use commands::{diff::run_diff, report::run_report, scan::run_scan, score::run_score};
use config::{Config, ConfigFormat};

#[derive(Parser)]
#[command(author, version, about = "QueryLens - SQL analysis and linting tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long, value_name = "PATH", default_value = ".querylens.yaml")]
        output: PathBuf,

        #[arg(short, long, value_enum, default_value_t = ConfigFormat::Yaml)]
        format: ConfigFormat,
    },
    Report {
        #[arg(value_name = "PATH", default_value = ".")]
        path: PathBuf,

        #[arg(short, long, value_enum, default_value_t = commands::report::ReportFormat::Table)]
        format: commands::report::ReportFormat,

        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,

        #[arg(long, value_name = "SECTION")]
        include: Vec<String>,

        #[arg(long, value_name = "LEVEL")]
        detail_level: Option<String>,
    },
    Scan {
        #[arg(value_name = "PATH", default_value = ".")]
        path: PathBuf,

        #[arg(short, long)]
        deep: bool,

        #[arg(long, value_name = "PATTERN")]
        exclude: Vec<String>,

        #[arg(short, long, value_enum, default_value_t = commands::scan::ScanFormat::Table)]
        format: commands::scan::ScanFormat,
    },
    Score {
        #[arg(value_name = "PATH", default_value = ".")]
        path: PathBuf,

        #[arg(long, value_name = "SCORE")]
        min_score: Option<u8>,

        #[arg(short, long)]
        detail: bool,

        #[arg(short, long, value_enum, default_value_t = commands::score::ScoreFormat::Json)]
        format: commands::score::ScoreFormat,
    },
    Diff {
        #[arg(value_name = "BEFORE")]
        before: PathBuf,

        #[arg(value_name = "AFTER")]
        after: PathBuf,

        #[arg(long, value_enum, default_value_t = commands::diff::DiffMode::Both)]
        mode: commands::diff::DiffMode,

        #[arg(short, long, value_enum, default_value_t = commands::diff::DiffFormat::Unified)]
        format: commands::diff::DiffFormat,

        #[arg(long)]
        show_unchanged: bool,
    },
    Examples,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { output, format } => {
            let config = Config::default();
            config.save_with_format(&output, format)?;
            println!("Configuration file created: {}", output.display());
        }
        Commands::Report { path, format, output, include, detail_level } => {
            let cfg = Config::load_optional(None)?;
            run_report(&path, &cfg, format, output.as_deref(), &include, detail_level.as_deref())?;
        }
        Commands::Scan { path, deep, exclude, format } => {
            let cfg = Config::load_optional(None)?;
            run_scan(&path, &cfg, deep, &exclude, format)?;
        }
        Commands::Score { path, min_score, detail, format } => {
            let cfg = Config::load_optional(None)?;
            run_score(&path, &cfg, min_score, detail, format)?;
        }
        Commands::Diff { before, after, mode, format, show_unchanged } => {
            run_diff(&before, &after, mode, format, show_unchanged)?;
        }
        Commands::Examples => {
            commands::examples::show_examples();
        }
    }

    Ok(())
}
