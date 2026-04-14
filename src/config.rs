use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub rules: HashMap<String, RuleConfig>,
    pub scan: ScanConfig,
    pub score: ScoreConfig,
    pub report: ReportConfig,
    pub dialect: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleConfig {
    pub enabled: bool,
    pub severity: String,
    #[serde(flatten)]
    pub params: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanConfig {
    pub patterns: Vec<String>,
    pub exclude: Vec<String>,
    pub max_file_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreConfig {
    pub weights: HashMap<String, f32>,
    pub thresholds: HashMap<String, u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportConfig {
    pub format: String,
    pub show_stats: bool,
    pub show_recommendations: bool,
    pub group_by: String,
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum ConfigFormat {
    Yaml,
    Json,
}

impl Default for Config {
    fn default() -> Self {
        let mut rules = HashMap::new();

        rules.insert(
            "hardcoded-credentials".to_string(),
            RuleConfig {
                enabled: true,
                severity: "error".to_string(),
                params: HashMap::new(),
            },
        );
        rules.insert(
            "select-star".to_string(),
            RuleConfig {
                enabled: true,
                severity: "warning".to_string(),
                params: HashMap::new(),
            },
        );
        rules.insert(
            "missing-else-case".to_string(),
            RuleConfig {
                enabled: true,
                severity: "warning".to_string(),
                params: HashMap::new(),
            },
        );
        rules.insert(
            "complex-case".to_string(),
            RuleConfig {
                enabled: true,
                severity: "info".to_string(),
                params: [("max_conditions".to_string(), serde_yaml::Value::from(5))]
                    .into_iter()
                    .collect(),
            },
        );

        Self {
            rules,
            scan: ScanConfig {
                patterns: vec!["*.sql".to_string()],
                exclude: vec!["**/migrations/**".to_string(), "**/node_modules/**".to_string()],
                max_file_size: 1_048_576,
            },
            score: ScoreConfig {
                weights: [
                    ("hardcoded_values".to_string(), 0.25),
                    ("case_complexity".to_string(), 0.20),
                    ("join_complexity".to_string(), 0.20),
                    ("filter_complexity".to_string(), 0.15),
                    ("maintainability".to_string(), 0.20),
                ]
                .into_iter()
                .collect(),
                thresholds: [
                    ("excellent".to_string(), 90),
                    ("good".to_string(), 75),
                    ("fair".to_string(), 60),
                    ("poor".to_string(), 40),
                ]
                .into_iter()
                .collect(),
            },
            report: ReportConfig {
                format: "table".to_string(),
                show_stats: true,
                show_recommendations: true,
                group_by: "category".to_string(),
            },
            dialect: "postgres".to_string(),
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let data = fs::read_to_string(path)
            .with_context(|| format!("Failed to read configuration file: {}", path.display()))?;

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("yaml") | Some("yml") => {
                let config: Config = serde_yaml::from_str(&data)?;
                Ok(config)
            }
            Some("json") => {
                let config: Config = serde_json::from_str(&data)?;
                Ok(config)
            }
            _ => Err(anyhow::anyhow!("Unsupported config extension: {}", path.display())),
        }
    }

    pub fn load_optional(config_path: Option<&Path>) -> Result<Self> {
        if let Some(path) = config_path {
            Self::load(path)
        } else {
            let candidates = [
                PathBuf::from(".querylens.yaml"),
                PathBuf::from(".querylens.yml"),
                PathBuf::from(".querylens.json"),
                PathBuf::from("querylens.yaml"),
                PathBuf::from("querylens.yml"),
                PathBuf::from("querylens.json"),
            ];
            for path in candidates.iter() {
                if path.exists() {
                    return Self::load(path);
                }
            }
            Ok(Config::default())
        }
    }

    pub fn save_with_format<P: AsRef<Path>>(&self, path: P, format: ConfigFormat) -> Result<()> {
        let path = path.as_ref();
        let data = match format {
            ConfigFormat::Yaml => serde_yaml::to_string(self)?,
            ConfigFormat::Json => serde_json::to_string_pretty(self)?,
        };
        fs::write(path, data).with_context(|| format!("Failed to write configuration file: {}", path.display()))?;
        Ok(())
    }
}
