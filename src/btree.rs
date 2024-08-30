//! Index set backed by a [`BTreeMap`].

use alloc::collections::btree_map::Entry;
use alloc::collections::BTreeMap;
#[cfg(feature = "serialize-borsh")]
use alloc::{format, string::ToString};
#[cfg(feature = "serialize-borsh")]
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
#[cfg(feature = "serialize-serde")]
use serde::{Deserialize, Serialize};

use super::calculate_map_and_set_indices;
use super::macros::{
    index_set_impl_extend, index_set_impl_from, index_set_impl_from_iterator, index_set_tests_for,
};
use super::storage;
use super::IndexSet;

/// Index set backed by a [`BTreeMap`].
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serialize-borsh",
    derive(BorshSerialize, BorshDeserialize, BorshSchema)
)]
#[cfg_attr(feature = "serialize-serde", derive(Serialize, Deserialize))]
pub struct BTreeIndexSet<S = u64> {
    /// Map of indices to bit vectors, containing the actual boolean
    /// values to be asserted.
    ///
    /// If the bit `B` is set, at the bit vector with index `S`, then
    /// the index `S::WIDTH * S + B` is in the set.
    bit_sets: BTreeMap<usize, S>,
}

impl<S> BTreeIndexSet<S> {
    /// Create a new [`BTreeIndexSet`].
    pub const fn new() -> Self {
        Self {
            bit_sets: BTreeMap::new(),
        }
    }

    /// Create a new [`BTreeIndexSet`] with the given capacity.
    ///
    /// ## Warning
    ///
    /// In the current implementation, this method is a stub.
    /// It doesn't actually provide any benefit over calling
    /// [`BTreeIndexSet::new`].
    #[inline]
    pub fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }
}

impl<S: storage::Storage> IndexSet for BTreeIndexSet<S> {
    #[inline]
    fn len(&self) -> usize {
        self.bit_sets
            .values()
            .map(|set| set.num_of_high_bits())
            .sum::<usize>()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.bit_sets.is_empty()
    }

    fn insert(&mut self, index: usize) {
        let (map_index, bit_set_index) = calculate_map_and_set_indices::<S>(index);
        let set = self.bit_sets.entry(map_index).or_insert(S::ZERO);
        *set |= S::from_usize(1 << bit_set_index);
    }

    fn remove(&mut self, index: usize) {
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

    fn contains(&self, index: usize) -> bool {
        let (map_index, bit_set_index) = calculate_map_and_set_indices::<S>(index);
        self.bit_sets
            .get(&map_index)
            .map(|&set| set & S::from_usize(1 << bit_set_index) != S::ZERO)
            .unwrap_or(false)
    }

    #[inline]
    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.bit_sets.iter().flat_map(|(&map_index, &set)| {
            (0..S::WIDTH).filter_map(move |bit_set_index| {
                let is_bit_set = (set & S::from_usize(1 << bit_set_index)) != S::ZERO;
                if is_bit_set {
                    Some(map_index * S::WIDTH + bit_set_index)
                } else {
                    None
                }
            })
        })
    }

    #[inline]
    fn union(&mut self, other: &BTreeIndexSet<S>) {
        for (&map_index, &other_set) in other.bit_sets.iter() {
            let set = self.bit_sets.entry(map_index).or_insert(S::ZERO);
            *set |= other_set;
        }
    }
}

index_set_impl_from!(crate::btree::BTreeIndexSet);
index_set_impl_from_iterator!(crate::btree::BTreeIndexSet);
index_set_impl_extend!(crate::btree::BTreeIndexSet);
index_set_tests_for!(crate::btree::BTreeIndexSet::<u64>);
