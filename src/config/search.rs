use serde::{Deserialize, Serialize};

use crate::utils::search::{search_match_exact, search_match_exact_lower, search_match_fuzzy};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub enum SearchMatchingStrategy {
    #[default]
    Fuzzy,
    Exact,
    ExactLowercase,
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
            SearchMatchingStrategy::Exact => Box::new(search_match_exact),
            SearchMatchingStrategy::ExactLowercase => Box::new(search_match_exact_lower),
        }
    }
}
