use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    error::Result,
    lint::{
        rules::{NoCache, Rule, RuleCache},
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

        for name in spec.sources.as_ref().unwrap_or(&HashMap::new()).keys() {
            if cache.common.sources.iter().filter(|s| *s == name).count() > 1 {
                results.push((
                    LintItem::Source,
                    name.clone(),
                    LintResult {
                        message: "source name is duplicated".to_string(),
                    },
                ));
            }
        }

        Ok(results)
    }
}
