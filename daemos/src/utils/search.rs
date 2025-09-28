use std::sync::LazyLock;

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};

pub static FUZZY_FINDER: LazyLock<SkimMatcherV2> = LazyLock::new(SkimMatcherV2::default);

pub fn search_match_fuzzy(search: &str, value: &str) -> bool {
    FUZZY_FINDER.fuzzy_match(value, search).is_some()
}

pub fn search_match_exact(search: &str, value: &str) -> bool {
    value.contains(search)
}

pub fn search_match_exact_lower(search: &str, value: &str) -> bool {
    value
        .to_ascii_lowercase()
        .contains(&search.to_ascii_lowercase())
}
