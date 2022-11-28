//! Index set backed by a [`Vec`].

use alloc::vec::Vec;

use super::calculate_map_and_set_indices;
use super::macros::index_set_tests_for;
use super::storage;

/// Index set backed by a [`Vec`].
#[derive(Default, Debug, Clone)]
pub struct VecIndexSet<S = u64> {
    /// Pairs of indices to bit vectors, containing the actual boolean
    /// values to be asserted.
    ///
    /// If the bit `B` is set, at the bit vector with index `S`, then
    /// the index `S::WIDTH * S + B` is in the set.
    bit_sets: Vec<(usize, S)>,
}

impl<S: storage::Storage> VecIndexSet<S> {
    /// Lookup the bit set at `map_index`, or initialize it
    /// with zero, if it doesn't exist.
    #[inline]
    fn lookup_or_zero(&mut self, map_index: usize) -> &mut S {
        let pair_index = self.lookup_or_initialize_pair(map_index);
        let (_, set) = &mut self.bit_sets[pair_index];
        set
    }

    /// Lookup the vec index of the bit set at `map_index`, or initialize it
    /// with zero, if it doesn't exist, returning the initialized index.
    #[inline]
    fn lookup_or_initialize_pair(&mut self, map_index: usize) -> usize {
        self.lookup_pair(map_index).map_or_else(
            |insert_at_index| {
                self.bit_sets.insert(insert_at_index, (map_index, S::ZERO));
                insert_at_index
            },
            |found_at_index| found_at_index,
        )
    }

    /// Lookup the vec index of the bit set at `map_index`.
    #[inline]
    fn lookup_pair(&self, map_index: usize) -> Result<usize, usize> {
        self.bit_sets.binary_search_by_key(&map_index, |&(i, _)| i)
    }

    /// Add a new index to this [`VecIndexSet`].
    pub fn insert(&mut self, index: usize) {
        let (map_index, bit_set_index) = calculate_map_and_set_indices::<S>(index);
        let set = self.lookup_or_zero(map_index);
        *set |= S::from_usize(1 << bit_set_index);
    }

    /// Remove an index from this [`VecIndexSet`].
    pub fn remove(&mut self, index: usize) {
        let (map_index, bit_set_index) = calculate_map_and_set_indices::<S>(index);
        let maybe_remove_index = self.lookup_pair(map_index).ok().and_then(|pair_index| {
            let (_, set) = &mut self.bit_sets[pair_index];
            *set &= !S::from_usize(1 << bit_set_index);
            if *set == S::ZERO {
                Some(pair_index)
            } else {
                None
            }
        });
        if let Some(pair_index) = maybe_remove_index {
            self.bit_sets.remove(pair_index);
        }
    }

    /// Check the presence of an index in this [`VecIndexSet`].
    pub fn contains(&self, index: usize) -> bool {
        let (map_index, bit_set_index) = calculate_map_and_set_indices::<S>(index);
        self.lookup_pair(map_index)
            .map(|pair_index| {
                let &(_, set) = &self.bit_sets[pair_index];
                set & S::from_usize(1 << bit_set_index) != S::ZERO
            })
            .unwrap_or(false)
    }

    /// Return an iterator over the indices in
    /// this [`VecIndexSet`], in ascending order.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.bit_sets.iter().flat_map(|&(map_index, set)| {
            (0..S::WIDTH).into_iter().flat_map(move |bit_set_index| {
                let is_bit_set = (set & S::from_usize(1 << bit_set_index)) != S::ZERO;
                is_bit_set.then(|| map_index * S::WIDTH + bit_set_index)
            })
        })
    }

    /// Merge two [`VecIndexSet`] instances.
    ///
    /// Corresponds to a mutating set union operation,
    /// between `self` and `other`.
    #[inline]
    pub fn union(&mut self, other: &VecIndexSet<S>) {
        // naive implementation
        for &(map_index, other_set) in other.bit_sets.iter() {
            let set = self.lookup_or_zero(map_index);
            *set |= other_set;
        }
    }
}

index_set_tests_for!(crate::vec::VecIndexSet::<u64>);
