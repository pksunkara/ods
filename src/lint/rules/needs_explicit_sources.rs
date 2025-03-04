use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    error::Result,
    lint::{
        rules::{CommonCache, NoCache, Rule, RuleCache},
        LintItem, LintLevel, LintResult,
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

    fn run(
        &self,
        cache: RuleCache<Self::Cache>,
        spec: &Spec,
    ) -> Result<Vec<(LintItem, String, LintResult)>> {
        let mut results = vec![];

        for (name, event) in spec.metrics.as_ref().unwrap_or(&HashMap::new()) {
            results.extend(self.check_sources(
                cache.common,
                &event.sources,
                LintItem::Metric,
                name,
            ));
        }

        for (name, event) in spec.pageviews.as_ref().unwrap_or(&HashMap::new()) {
            results.extend(self.check_sources(
                cache.common,
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
        cache: &CommonCache,
        sources: &Option<Vec<String>>,
        ty: LintItem,
        name: &str,
    ) -> Vec<(LintItem, String, LintResult)> {
        let mut results = vec![];

        if let Some(sources) = sources {
            for source in sources {
                if !cache.sources.contains(source) {
                    results.push((
                        ty,
                        name.to_owned(),
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
