//! Set data structures optimized to store [`usize`] values.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod btree;
mod macros;
mod storage;
pub mod vec;

/// Public interface of any index set implementation.
pub trait IndexSet {
    /// Return the number of [`usize`] values present
    /// in this [`IndexSet`].
    fn len(&self) -> usize;

    /// Checks if this [`IndexSet`] has no inner indexes
    /// stored within.
    fn is_empty(&self) -> bool;

    /// Add a new index to this [`IndexSet`].
    fn insert(&mut self, index: usize);

    /// Remove an index from this [`IndexSet`].
    fn remove(&mut self, index: usize);

    /// Check the presence of an index in this [`IndexSet`].
    fn contains(&self, index: usize) -> bool;

    /// Return an iterator over the indices in
    /// this [`IndexSet`], in ascending order.
    fn iter(&self) -> impl Iterator<Item = usize> + '_;

    /// Merge two [`IndexSet`] instances.
    ///
    /// Corresponds to a mutating set union operation,
    /// between `self` and `other`.
    fn union(&mut self, other: &Self);

    /// Attempt to reserve space for the specified
    /// number of additional [`usize`] elements.
    fn reserve(&mut self, _size: usize) {
        // NOOP
    }
}

#[inline]
fn safe_iter_reserve_cap<I>(iter: &I) -> usize
where
    I: Iterator,
{
    let (min_cap, _) = iter.size_hint();

    // NB: guard against malicious impls
    // with a reasonable higher bound
    min_cap.min(256)
}

#[inline]
const fn calculate_map_and_set_indices<S>(index: usize) -> (usize, usize)
where
    S: storage::Storage,
{
    // these let exprs will get optimized into a single op,
    // since they're in sequence, which is nice
    let map_index = index / S::WIDTH;
    let bit_set_index = index % S::WIDTH;

    (map_index, bit_set_index)
}
