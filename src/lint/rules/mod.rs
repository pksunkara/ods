use std::fmt::Debug as FmtDebug;

use clap::ValueEnum;
use indexmap::IndexMap;
use serde::Deserialize;
use tracing::{instrument, trace};

use crate::{
    error::Result,
    lint::{
        LintItem, LintLevel, LintResult,
        rules::cache::{CommonCache, RuleCache},
    },
    schema::spec::Spec,
};

#[macro_use]
mod macro_def;

mod cache;

rules! {
    needs_explicit_sources,
    needs_metric_description,
    needs_source_description,
    no_duplicate_metrics,
    no_duplicate_pageviews,
    no_duplicate_sources,
    uses_name_case,
}

impl Rules {
    #[instrument(name = "pre_compute", skip_all)]
    pub(super) fn pre_compute(specs: Vec<&Spec>) -> Result<RulesCache> {
        let mut cache = RulesCache::default();

        for spec in &specs {
            cache._common.pre_compute(spec)?;
        }

        for rule in Rules::value_variants() {
            trace!("Pre-computing for rule: {}", rule);

            for spec in &specs {
                cache.pre_compute_rule(rule, spec)?;
            }
        }

        Ok(cache)
    }

    #[allow(clippy::type_complexity)]
    #[instrument(name = "run", skip_all)]
    pub(super) fn run(
        cache: &RulesCache,
        lint_file_config: Option<&RulesConfig>,
        spec: &Spec,
    ) -> Result<IndexMap<LintItem, IndexMap<String, Vec<(LintLevel, LintResult)>>>> {
        let mut all_results = IndexMap::new();

        // Merge common and spec lint configurations
        let rules_config = spec.lint.as_ref().cloned().map_or(
            lint_file_config.cloned().unwrap_or_default(),
            |spec_config| {
                lint_file_config.map_or(spec_config.clone(), |common_config| {
                    spec_config.base_upon(common_config)
                })
            },
        );

        for rule in Rules::value_variants() {
            trace!("Running rule: {}", rule);
            let (level, results) = rules_config.run_rule(rule, cache, spec)?;

            for (ty, name, result) in results {
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
    type Cache;

    fn level(&self) -> LintLevel {
        LintLevel::Warning
    }

    fn pre_compute(_: &mut Self::Cache, _: &Spec) -> Result<()> {
        Ok(())
    }

    fn run(
        &self,
        cache: RuleCache<Self::Cache>,
        spec: &Spec,
    ) -> Result<Vec<(LintItem, String, LintResult)>>;
}

type NoCache = ();
