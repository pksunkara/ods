use std::{
    fmt::{Display as FmtDisplay, Formatter, Result as FmtResult},
    path::{absolute, PathBuf},
};

use anstream::println;
use clap::Parser;
use eyre::eyre;
use owo_colors::OwoColorize;
use proc_exit::Code;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use tracing::{debug, instrument, trace};

use crate::{
    error::{exit, Result},
    lint::rules::Rules,
    schema::SchemaOpt,
};

pub mod rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum LintLevel {
    Off,

    #[serde(rename = "warn")]
    Warning,

    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum LintItem {
    Metric,
    Source,
}

impl FmtDisplay for LintItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            to_string(self)
                .expect("Failed to serialize LintItem")
                .trim_matches('"')
        )
    }
}

#[derive(Debug)]
struct LintResult {
    message: String,
}

/// Lint plan
#[derive(Debug, Parser)]
pub struct Lint {
    #[clap(flatten)]
    schema: SchemaOpt,

    /// File paths in the plan folder to lint (defaults to all)
    files: Vec<PathBuf>,

    /// Exit with a zero code even on lint errors
    #[clap(long)]
    no_fail: bool,
}

impl Lint {
    #[instrument(name = "lint", skip_all)]
    pub fn run(self) -> Result {
        let mut warnings = 0;
        let mut errors = 0;

        let files = self.schema.load()?;

        let plan_path = absolute(&self.schema.plan)?;

        // Filter files to lint based on user input
        let selected = if self.files.is_empty() {
            files
        } else {
            let selected_files = self
                .files
                .into_iter()
                .map(|file| {
                    let file_path = absolute(&file)?;

                    if !file.exists() {
                        return Err(eyre!("unable to find {}", file.display()));
                    }

                    let relative_file_path = file_path
                        .strip_prefix(&plan_path)
                        .map_err(|_| eyre!("plan does not contain {}", file.display()))?
                        .to_string_lossy()
                        .to_string();

                    if !files.contains_key(&relative_file_path) {
                        return Err(eyre!("unable to find {}", file.display()));
                    }

                    Ok(relative_file_path)
                })
                .collect::<Result<Vec<_>>>()?;

            files
                .into_iter()
                .filter(|(k, _)| selected_files.contains(k))
                .collect()
        };

        // Check if lint config file exists
        let lint_file_config = selected
            .iter()
            .find(|(name, _)| ["lints.json", "lints.yaml", "lints.yml"].contains(&name.as_str()))
            .and_then(|(_, spec)| spec.lint.as_ref());

        // Compute cache
        let cache = Rules::pre_compute(selected.iter().map(|(_, spec)| spec).collect())?;

        // Lint each file
        for (name, spec) in &selected {
            debug!("Linting file: {}", name);
            let spec_results = Rules::run(&cache, lint_file_config, spec)?;

            if spec_results.is_empty() {
                trace!("No issues found in file: {}", name);
                continue;
            }

            println!("\n{}", name.magenta());

            for (ty, ty_results) in spec_results {
                for (name, results) in ty_results {
                    println!("  {} {}", name.blue(), format!("({ty})").cyan());

                    for (level, result) in results {
                        match level {
                            LintLevel::Off => {}
                            LintLevel::Warning => {
                                warnings += 1;
                                println!("    {} {}", " warn".yellow(), result.message);
                            }
                            LintLevel::Error => {
                                errors += 1;
                                println!("    {} {}", "error".red(), result.message);
                            }
                        }
                    }
                }
            }
        }

        if warnings > 0 || errors > 0 {
            println!(
                "\n{} errors, {} warnings\n",
                errors.red().bold(),
                warnings.yellow().bold()
            );
        }

        if errors > 0 && !self.no_fail {
            exit(Code::FAILURE);
        }

        Ok(())
    }
}
