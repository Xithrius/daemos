use std::collections::BTreeSet;

use rand::Rng;

pub fn filtered_random_index(len: usize, excluded: &BTreeSet<usize>) -> Option<usize> {
    if excluded.len() >= len {
        return None;
    }

    let valid_count = len - excluded.len();
    let mut rng = rand::rng();
    let target = rng.random_range(0..valid_count);

    // Walk through the index space, adjusting for exclusions
    let mut offset = 0;
    for &ex in excluded {
        if ex - offset > target {
            break;
        }
        offset += 1;
    }

    Some(target + offset)
}
