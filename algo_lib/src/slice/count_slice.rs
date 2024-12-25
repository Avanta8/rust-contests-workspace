use crate::collections::fxhash::FxHashMap;

pub fn count_slice<T: Eq + std::hash::Hash>(v: &[T]) -> FxHashMap<&T, usize> {
    let mut counts = FxHashMap::default();

    for item in v {
        *counts.entry(item).or_insert(0) += 1;
    }

    counts
}
