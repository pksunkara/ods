macro_rules! rules {
    ($($rule:ident,)+) => {
        $(mod $rule;)+

        #[allow(non_camel_case_types)]
        #[derive(Clone, ValueEnum)]
        pub(super) enum Rules {
            $($rule,)+
        }

        impl std::fmt::Display for Rules {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(Rules::$rule => write!(f, stringify!($rule)),)+
                }
            }
        }

        paste::paste! {
            $(
                #[allow(non_camel_case_types)]
                #[derive(Debug, Clone, Default, Deserialize)]
                struct [<$rule _config>] {
                    level: Option<LintLevel>,
                    #[serde(flatten)]
                    config: $rule::Config,
                }
            )+

            #[derive(Debug, Clone, Default, Deserialize)]
            pub struct RulesConfig {
                $($rule: Option<[<$rule _config>]>,)+
            }

            impl RulesConfig {
                pub(super) fn base_upon(self, common: &Self) -> Self {
                    Self {
                        $($rule: self.$rule.as_ref().map_or(common.$rule.clone(), |self_rule| {
                            common.$rule.as_ref().map_or(Some(self_rule.clone()), |common_rule| {
                                Some([<$rule _config>] {
                                    level: self_rule.level.or(common_rule.level),
                                    config: self_rule.config.clone(),
                                })
                            })
                        }),)+
                    }
                }
            }

            #[derive(Debug, Default)]
            pub(super) struct RulesCache {
                $($rule: <$rule::Config as Rule>::Cache,)+
            }
        }

        impl RulesCache {
            pub(super) fn pre_compute_rule(&mut self, rule: &Rules, spec: &Spec) -> Result<()> {
                match rule {
                    $(Rules::$rule => <$rule::Config as Rule>::pre_compute(&mut self.$rule, spec)?,)+
                }

                Ok(())
            }
        }

        impl RulesConfig {
            pub(super) fn run_rule(&self, rule: &Rules, cache: &RulesCache, spec: &Spec) -> Result<(LintItem, LintLevel, Vec<(String, LintResult)>)> {
                match rule {
                    $(Rules::$rule => {
                        let config = self
                            .$rule
                            .as_ref()
                            .cloned()
                            .unwrap_or_default();

                        let level = config.level.unwrap_or_else(|| config.config.level());
                        let ty = config.config.ty();

                        if level == LintLevel::Off {
                            return Ok((ty, level, vec![]));
                        }

                        let results = config.config.run(&cache.$rule, spec)?;

                        return Ok((ty, level, results));
                    })+
                }
            }
        }
    };
}
