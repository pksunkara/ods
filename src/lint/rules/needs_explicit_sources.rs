use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    error::Result,
    lint::{
        rules::{NoCache, Rule},
        LintItem, LintLevel, LintResult,
    },
    schema::spec::{Source, Spec},
};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {}

impl Rule for Config {
    type Cache = NoCache;

    fn level(&self) -> LintLevel {
        LintLevel::Error
    }

    fn run(&self, _: &Self::Cache, spec: &Spec) -> Result<Vec<(LintItem, String, LintResult)>> {
        let mut results = vec![];
        let empty = HashMap::new();

        // TODO: Use the global sources
        let defined_sources = spec.sources.as_ref().unwrap_or(&empty);

        for (name, event) in spec.metrics.as_ref().unwrap_or(&HashMap::new()) {
            results.extend(self.check_sources(
                defined_sources,
                &event.sources,
                LintItem::Metric,
                name,
            ));
        }

        for (name, event) in spec.pageviews.as_ref().unwrap_or(&HashMap::new()) {
            results.extend(self.check_sources(
                defined_sources,
                &event.sources,
                LintItem::Pageview,
                name,
            ));
        }

        Ok(results)
    }
}

impl Config {
    fn check_sources(
        &self,
        defined_sources: &HashMap<String, Source>,
        sources: &Option<Vec<String>>,
        ty: LintItem,
        name: &String,
    ) -> Vec<(LintItem, String, LintResult)> {
        let mut results = vec![];

        if let Some(sources) = sources {
            for source in sources {
                if !defined_sources.contains_key(source) {
                    results.push((
                        ty,
                        name.clone(),
                        LintResult {
                            message: format!("source `{source}` is not defined"),
                        },
                    ));
                }
            }
        }

        results
    }
}
