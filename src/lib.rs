//! Set data structure optimized to store [`usize`] values.

pub mod btree;
mod macros;
mod storage;

/// Set data structure optimized to store [`usize`] values.
pub type IndexSet<S = u64> = btree::BTreeIndexSet<S>;

#[inline]
const fn calculate_map_and_set_indices<S>(index: usize) -> (usize, usize)
where
    S: storage::Storage,
{
    // these let exprs will get optimized into a single op,
    // since they're ordered in sequence, which is nice
    let map_index = index / S::WIDTH;
    let bit_set_index = index % S::WIDTH;

    (map_index, bit_set_index)
}
