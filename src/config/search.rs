use std::fmt;

use serde::{Deserialize, Serialize};

use crate::utils::search::{search_match_exact, search_match_exact_lower, search_match_fuzzy};

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SearchMatchingStrategy {
    #[default]
    Fuzzy,
    ContainsExact,
    ContainsLowercase,
}

impl fmt::Display for SearchMatchingStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            SearchMatchingStrategy::Fuzzy => "Fuzzy",
            SearchMatchingStrategy::ContainsExact => "Contains (exact)",
            SearchMatchingStrategy::ContainsLowercase => "Contains (lowercase)",
        };

        write!(f, "{label}")
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct SearchConfig {
    pub strategy: SearchMatchingStrategy,
}

pub type MatcherFn = Box<dyn Fn(&str, &str) -> bool + Send + Sync>;

impl SearchMatchingStrategy {
    pub fn get_matcher(&self) -> MatcherFn {
        match self {
            SearchMatchingStrategy::Fuzzy => Box::new(search_match_fuzzy),
            SearchMatchingStrategy::ContainsExact => Box::new(search_match_exact),
            SearchMatchingStrategy::ContainsLowercase => Box::new(search_match_exact_lower),
        }
    }
}
