use std::collections::HashMap;

use super::types::MatchMethod;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MatchedIndex {
    pub before: Option<usize>,
    pub after: Option<usize>,
    pub method: Option<MatchMethod>,
}

pub fn match_items<T, P, F>(
    before: &[T],
    after: &[T],
    primary_key: P,
    fallback_key: F,
    fallback_method: MatchMethod,
) -> Vec<MatchedIndex>
where
    P: Fn(&T) -> String,
    F: Fn(&T) -> Option<String>,
{
    let mut matches = Vec::new();
    let mut before_used = vec![false; before.len()];
    let mut after_used = vec![false; after.len()];

    let mut after_by_primary = HashMap::<String, usize>::new();
    for (index, item) in after.iter().enumerate() {
        after_by_primary.insert(primary_key(item), index);
    }

    for (before_index, item) in before.iter().enumerate() {
        if let Some(&after_index) = after_by_primary.get(&primary_key(item)) {
            if !after_used[after_index] {
                before_used[before_index] = true;
                after_used[after_index] = true;
                matches.push(MatchedIndex {
                    before: Some(before_index),
                    after: Some(after_index),
                    method: Some(MatchMethod::ObjectId),
                });
            }
        }
    }

    let mut before_fallback_counts = HashMap::<String, usize>::new();
    let mut after_fallback_counts = HashMap::<String, usize>::new();
    let mut after_by_fallback = HashMap::<String, usize>::new();

    for (index, item) in before.iter().enumerate() {
        if before_used[index] {
            continue;
        }
        if let Some(key) = fallback_key(item) {
            *before_fallback_counts.entry(key).or_default() += 1;
        }
    }

    for (index, item) in after.iter().enumerate() {
        if after_used[index] {
            continue;
        }
        if let Some(key) = fallback_key(item) {
            *after_fallback_counts.entry(key.clone()).or_default() += 1;
            after_by_fallback.insert(key, index);
        }
    }

    for (before_index, item) in before.iter().enumerate() {
        if before_used[before_index] {
            continue;
        }

        let Some(key) = fallback_key(item) else {
            continue;
        };

        if before_fallback_counts.get(&key) != Some(&1)
            || after_fallback_counts.get(&key) != Some(&1)
        {
            continue;
        }

        let Some(&after_index) = after_by_fallback.get(&key) else {
            continue;
        };

        before_used[before_index] = true;
        after_used[after_index] = true;
        matches.push(MatchedIndex {
            before: Some(before_index),
            after: Some(after_index),
            method: Some(fallback_method),
        });
    }

    for (index, used) in before_used.iter().enumerate() {
        if !used {
            matches.push(MatchedIndex {
                before: Some(index),
                after: None,
                method: None,
            });
        }
    }

    for (index, used) in after_used.iter().enumerate() {
        if !used {
            matches.push(MatchedIndex {
                before: None,
                after: Some(index),
                method: None,
            });
        }
    }

    matches
}
