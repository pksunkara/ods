use std::collections::HashMap;

use crate::{error::Result, schema::spec::Spec};

#[derive(Debug, Default)]
pub(super) struct CommonCache {
    pub(super) sources: Vec<String>,
}

impl CommonCache {
    pub(super) fn pre_compute(&mut self, spec: &Spec) -> Result<()> {
        self.sources.extend(
            spec.sources
                .as_ref()
                .unwrap_or(&HashMap::new())
                .iter()
                .map(|(name, _)| name.clone()),
        );

        Ok(())
    }
}

pub(super) struct RuleCache<'a, T> {
    pub(super) common: &'a CommonCache,
    pub(super) rule: &'a T,
}
