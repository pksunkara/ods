use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    error::Result,
    lint::{
        rules::{NoCache, Rule},
        LintResult,
    },
    schema::spec::Spec,
};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {}

impl Rule for Config {
    type Cache = NoCache;

    fn run(&self, _: &Self::Cache, spec: &Spec) -> Result<Vec<(String, LintResult)>> {
        let mut results = vec![];

        for (name, event) in spec.metrics.as_ref().unwrap_or(&HashMap::new()) {
            if event.description.is_none() {
                results.push((
                    name.clone(),
                    LintResult {
                        message: "description is missing".to_string(),
                    },
                ));
            }
        }

        Ok(results)
    }
}
