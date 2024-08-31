//! Index set backed by a [`Vec`].

use alloc::vec::Vec;
#[cfg(feature = "serialize-borsh")]
use alloc::{format, string::ToString};
#[cfg(feature = "serialize-borsh")]
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
#[cfg(feature = "serialize-serde")]
use serde::{Deserialize, Serialize};

use super::calculate_map_and_set_indices;
use super::macros::*;
use super::storage;
use super::IndexSet;

#[cfg(feature = "serialize-borsh")]
mod borsh_deserialize {
    use super::*;

    /// Deserialize a [`VecIndexSet`] from borsh data.
    pub fn from<R, S>(reader: &mut R) -> Result<Vec<(usize, S)>, borsh::io::Error>
    where
        R: borsh::io::Read,
        S: borsh::de::BorshDeserialize,
    {
        let bit_sets: Vec<(usize, S)> = borsh::BorshDeserialize::deserialize_reader(reader)?;
        for window in bit_sets.windows(2) {
            let &[(a, _), (b, _)] = window else {
                unreachable!()
            };
            if a > b {
                return Err(borsh::io::Error::new(
                    borsh::io::ErrorKind::Other,
                    "VecIndexSet should have been sorted",
                ));
            }
        }
        Ok(bit_sets)
    }
}

/// Index set backed by a [`Vec`].
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serialize-borsh",
    derive(BorshSerialize, BorshDeserialize, BorshSchema)
)]
#[cfg_attr(feature = "serialize-serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct VecIndexSet<S = u64> {
    /// Pairs of indices to bit vectors, containing the actual boolean
    /// values to be asserted.
    ///
    /// If the bit `B` is set, at the bit vector with index `S`, then
    /// the index `S::WIDTH * S + B` is in the set.
    #[cfg_attr(
        feature = "serialize-borsh",
        borsh(deserialize_with = "borsh_deserialize::from")
    )]
    bit_sets: Vec<(usize, S)>,
}

impl<S> VecIndexSet<S> {
    /// Create a new [`VecIndexSet`].
    pub const fn new() -> Self {
        Self {
            bit_sets: Vec::new(),
        }
    }

    /// Create a new [`VecIndexSet`] with the given capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bit_sets: Vec::with_capacity(capacity),
        }
    }
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
        self.lookup_pair(map_index)
            .unwrap_or_else(|insert_at_index| {
                self.bit_sets.insert(insert_at_index, (map_index, S::ZERO));
                insert_at_index
            })
    }

    /// Lookup the vec index of the bit set at `map_index`.
    #[inline]
    fn lookup_pair(&self, map_index: usize) -> Result<usize, usize> {
        self.bit_sets.binary_search_by_key(&map_index, |&(i, _)| i)
    }
}

impl<S: storage::Storage> IndexSet for VecIndexSet<S> {
    #[inline]
    fn len(&self) -> usize {
        self.bit_sets
            .iter()
            .map(|(_, set)| set.num_of_high_bits())
            .sum::<usize>()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.bit_sets.is_empty()
    }

    fn insert(&mut self, index: usize) {
        let (map_index, bit_set_index) = calculate_map_and_set_indices::<S>(index);
        let set = self.lookup_or_zero(map_index);
        *set |= S::from_usize(1 << bit_set_index);
    }

    fn remove(&mut self, index: usize) {
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

    fn contains(&self, index: usize) -> bool {
        let (map_index, bit_set_index) = calculate_map_and_set_indices::<S>(index);
        self.lookup_pair(map_index)
            .map(|pair_index| {
                let &(_, set) = &self.bit_sets[pair_index];
                set & S::from_usize(1 << bit_set_index) != S::ZERO
            })
            .unwrap_or(false)
    }

    #[inline]
    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.bit_sets.iter().flat_map(|&(map_index, set)| {
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
    fn union(&mut self, other: &VecIndexSet<S>) {
        // naive implementation
        for &(map_index, other_set) in other.bit_sets.iter() {
            let set = self.lookup_or_zero(map_index);
            *set |= other_set;
        }
    }

    #[inline]
    fn reserve(&mut self, size: usize) {
        self.bit_sets.reserve(size);
    }
}

index_set_impl_from!(crate::vec::VecIndexSet);
index_set_impl_from_iterator!(crate::vec::VecIndexSet);
index_set_impl_extend!(crate::vec::VecIndexSet);
index_set_tests!(crate::vec::VecIndexSet);
