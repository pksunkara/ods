use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    error::Result,
    lint::{rules::Rule, LintItem, LintLevel, LintResult},
    schema::spec::Spec,
};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {}

#[derive(Debug, Default)]
pub struct Cache {
    metrics: Vec<String>,
}

impl Rule for Config {
    type Cache = Cache;

    fn level(&self) -> LintLevel {
        LintLevel::Error
    }

    fn pre_compute(cache: &mut Self::Cache, spec: &Spec) -> Result<()> {
        cache.metrics.extend(
            spec.metrics
                .as_ref()
                .unwrap_or(&HashMap::new())
                .iter()
                .map(|(name, _)| name.clone()),
        );

        Ok(())
    }

    fn run(&self, cache: &Self::Cache, spec: &Spec) -> Result<Vec<(LintItem, String, LintResult)>> {
        let mut results = vec![];

        for name in spec.metrics.as_ref().unwrap_or(&HashMap::new()).keys() {
            if cache.metrics.iter().filter(|s| *s == name).count() > 1 {
                results.push((
                    LintItem::Metric,
                    name.clone(),
                    LintResult {
                        message: "metric name is duplicated".to_string(),
                    },
                ));
            }
        }

        Ok(results)
    }
}
