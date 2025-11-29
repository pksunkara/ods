use std::{
    collections::HashMap,
    fmt::{Display as FmtDisplay, Formatter, Result as FmtResult},
};

use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToTitleCase, ToTrainCase,
};
use serde::Deserialize;

use crate::{
    error::Result,
    commands::lint::{
        rules::{NoCache, Rule, RuleCache},
        LintItem, LintLevel, LintResult,
    },
    schema::spec::Spec,
};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Case {
    TitleCase,
    LowerCase,
    UpperCase,
    CamelCase,
    PascalCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
    TrainCase,
}

impl FmtDisplay for Case {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Case::TitleCase => "Title Case",
                Case::LowerCase => "lower case",
                Case::UpperCase => "UPPER CASE",
                Case::CamelCase => "camelCase",
                Case::PascalCase => "PascalCase",
                Case::SnakeCase => "snake_case",
                Case::ScreamingSnakeCase => "SCREAMING_SNAKE_CASE",
                Case::KebabCase => "kebab-case",
                Case::ScreamingKebabCase => "SCREAMING-KEBAB-CASE",
                Case::TrainCase => "Train-Case",
            }
        )
    }
}

impl Case {
    fn convert(&self, name: &str) -> String {
        match self {
            Case::LowerCase => name.to_title_case().to_lowercase(),
            Case::UpperCase => name.to_title_case().to_uppercase(),
            Case::TitleCase => name.to_title_case(),
            Case::CamelCase => name.to_lower_camel_case(),
            Case::PascalCase => name.to_pascal_case(),
            Case::SnakeCase => name.to_snake_case(),
            Case::ScreamingSnakeCase => name.to_shouty_snake_case(),
            Case::KebabCase => name.to_kebab_case(),
            Case::ScreamingKebabCase => name.to_shouty_kebab_case(),
            Case::TrainCase => name.to_train_case(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    pub metric: Option<Case>,
    pub pageview: Option<Case>,
    // TODO: property
}

impl Rule for Config {
    type Cache = NoCache;

    fn level(&self) -> LintLevel {
        LintLevel::Error
    }

    fn run(
        &self,
        _: RuleCache<Self::Cache>,
        spec: &Spec,
    ) -> Result<Vec<(LintItem, String, LintResult)>> {
        let mut results = vec![];

        let metric_case = self.metric.as_ref().unwrap_or(&Case::TitleCase);
        let pageview_case = self.pageview.as_ref().unwrap_or(&Case::TitleCase);

        for name in spec.metrics.as_ref().unwrap_or(&HashMap::new()).keys() {
            if &metric_case.convert(name) != name {
                results.push((
                    LintItem::Metric,
                    name.clone(),
                    LintResult {
                        message: format!("name is not in {metric_case}"),
                    },
                ));
            }
        }

        for name in spec.pageviews.as_ref().unwrap_or(&HashMap::new()).keys() {
            if &pageview_case.convert(name) != name {
                results.push((
                    LintItem::Pageview,
                    name.clone(),
                    LintResult {
                        message: format!("name is not in {pageview_case}"),
                    },
                ));
            }
        }

        Ok(results)
    }
}
