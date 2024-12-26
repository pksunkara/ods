use std::fmt::Debug as FmtDebug;

use clap::ValueEnum;
use indexmap::IndexMap;
use serde::Deserialize;
use tracing::{instrument, trace};

use crate::{
    error::Result,
    lint::{LintItem, LintLevel, LintResult},
    schema::spec::Spec,
};

#[macro_use]
mod macro_def;

rules! {
    needs_description,
    needs_explicit_sources,
    needs_source_description,
}

impl Rules {
    #[allow(clippy::type_complexity)]
    #[instrument(name = "run", skip_all)]
    pub(super) fn run(
        spec: &Spec,
        common_rules_config: Option<&RulesConfig>,
    ) -> Result<IndexMap<LintItem, IndexMap<String, Vec<(LintLevel, LintResult)>>>> {
        let mut all_results = IndexMap::new();

        // Merge common and spec lint configurations
        let rules_config = spec.lint.as_ref().cloned().map_or(
            common_rules_config.cloned().unwrap_or_default(),
            |spec_config| {
                common_rules_config.map_or(spec_config.clone(), |common_config| {
                    spec_config.base_upon(common_config)
                })
            },
        );

        for rule in Rules::value_variants() {
            trace!("Running rule: {}", rule);
            let (ty, level, results) = rules_config.run_rule(rule, spec)?;

            for (name, result) in results {
                all_results
                    .entry(ty)
                    .or_insert_with(IndexMap::new)
                    .entry(name)
                    .or_insert_with(Vec::new)
                    .push((level, result));
            }
        }

        Ok(all_results)
    }
}

trait Rule: FmtDebug + Clone + Default + for<'de> Deserialize<'de> {
    fn level(&self) -> LintLevel {
        LintLevel::Warning
    }

    fn ty(&self) -> LintItem {
        LintItem::Metric
    }

    fn run(&self, spec: &Spec) -> Result<Vec<(String, LintResult)>>;
}
