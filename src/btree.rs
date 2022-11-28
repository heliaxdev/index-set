//! Index set backed by a [`BTreeMap`].

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use super::calculate_map_and_set_indices;
use super::macros::index_set_tests_for;
use super::storage;

#[derive(Default, Debug, Clone)]
pub struct BTreeIndexSet<S = u64> {
    /// Map of indices to bit vectors, containing the actual boolean
    /// values to be asserted.
    ///
    /// If the bit `B` is set, at the bit vector with index `S`, then
    /// the index `S::WIDTH * S + B` is in the set.
    bit_sets: BTreeMap<usize, S>,
}

impl<S: storage::Storage> BTreeIndexSet<S> {
    /// Add a new index to this [`BTreeIndexSet`].
    pub fn insert(&mut self, index: usize) {
        let (map_index, bit_set_index) = calculate_map_and_set_indices::<S>(index);
        let set = self.bit_sets.entry(map_index).or_insert(S::ZERO);
        *set |= S::from_usize(1 << bit_set_index);
    }

    /// Remove an index from this [`BTreeIndexSet`].
    pub fn remove(&mut self, index: usize) {
        let (map_index, bit_set_index) = calculate_map_and_set_indices::<S>(index);
        let entry = self.bit_sets.entry(map_index).and_modify(|set| {
            *set &= !S::from_usize(1 << bit_set_index);
        });
        match entry {
            Entry::Occupied(e) if *e.get() == S::ZERO => {
                e.remove();
            }
            _ => {}
        }
    }

    /// Check the presence of an index in this [`BTreeIndexSet`].
    pub fn contains(&self, index: usize) -> bool {
        let (map_index, bit_set_index) = calculate_map_and_set_indices::<S>(index);
        self.bit_sets
            .get(&map_index)
            .map(|&set| set & S::from_usize(1 << bit_set_index) != S::ZERO)
            .unwrap_or(false)
    }

    /// Return an iterator over the indices in
    /// this [`BTreeIndexSet`], in ascending order.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.bit_sets.iter().flat_map(|(&map_index, &set)| {
            (0..S::WIDTH).into_iter().flat_map(move |bit_set_index| {
                let is_bit_set = (set & S::from_usize(1 << bit_set_index)) != S::ZERO;
                is_bit_set.then(|| map_index * S::WIDTH + bit_set_index)
            })
        })
    }

    /// Merge two [`BTreeIndexSet`] instances.
    ///
    /// Corresponds to a mutating set union operation,
    /// between `self` and `other`.
    #[inline]
    pub fn union(&mut self, other: &BTreeIndexSet<S>) {
        for (&map_index, &other_set) in other.bit_sets.iter() {
            let set = self.bit_sets.entry(map_index).or_insert(S::ZERO);
            *set |= other_set;
        }
    }
}

index_set_tests_for!(crate::btree::BTreeIndexSet::<u64>);
