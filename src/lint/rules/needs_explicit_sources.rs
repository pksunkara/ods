use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    error::Result,
    lint::{
        rules::{NoCache, Rule},
        LintLevel, LintResult,
    },
    schema::spec::Spec,
};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {}

impl Rule for Config {
    type Cache = NoCache;

    fn level(&self) -> LintLevel {
        LintLevel::Error
    }

    fn run(&self, _: &Self::Cache, spec: &Spec) -> Result<Vec<(String, LintResult)>> {
        let mut results = vec![];
        let empty = HashMap::new();

        let defined_sources = spec.sources.as_ref().unwrap_or(&empty);

        for (name, event) in spec.metrics.as_ref().unwrap_or(&HashMap::new()) {
            if let Some(sources) = &event.sources {
                for source in sources {
                    if !defined_sources.contains_key(source) {
                        results.push((
                            name.clone(),
                            LintResult {
                                message: format!("source `{source}` is not defined"),
                            },
                        ));
                    }
                }
            }
        }

        Ok(results)
    }
}
