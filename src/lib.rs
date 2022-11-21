//! Set data structure optimized to store [`usize`] values.

mod storage;

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

/// Set data structure optimized to store [`usize`] values.
#[derive(Default, Debug, Clone)]
pub struct IndexSet<S = u64> {
    /// Map of indices to bit vectors, containing the actual boolean
    /// values to be asserted.
    ///
    /// If the bit `B` is set, at the bit vector with index `S`, then
    /// the index `S::WIDTH * S + B` is in the set.
    bit_sets: BTreeMap<usize, S>,
}

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

impl<S: storage::Storage> IndexSet<S> {
    /// Add a new index to this [`IndexSet`].
    pub fn insert(&mut self, index: usize) {
        let (map_index, bit_set_index) = calculate_map_and_set_indices::<S>(index);
        let set = self.bit_sets.entry(map_index).or_insert(S::ZERO);
        *set |= S::from_usize(1 << bit_set_index);
    }

    /// Remove an index from this [`IndexSet`].
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

    /// Check the presence of an index in this [`IndexSet`].
    pub fn contains(&self, index: usize) -> bool {
        let (map_index, bit_set_index) = calculate_map_and_set_indices::<S>(index);
        self.bit_sets
            .get(&map_index)
            .map(|&set| set & S::from_usize(1 << bit_set_index) != S::ZERO)
            .unwrap_or(false)
    }

    /// Return an iterator over the indices in
    /// this [`IndexSet`], in ascending order.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.bit_sets.iter().flat_map(|(&map_index, &set)| {
            (0..S::WIDTH).into_iter().flat_map(move |bit_set_index| {
                let is_bit_set = (set & S::from_usize(1 << bit_set_index)) != S::ZERO;
                is_bit_set.then_some(map_index * S::WIDTH + bit_set_index)
            })
        })
    }

    /// Merge two [`IndexSet`] instances.
    ///
    /// Corresponds to a mutating set union operation,
    /// between `self` and `other`.
    #[inline]
    pub fn union(&mut self, other: &IndexSet<S>) {
        for (&map_index, &other_set) in other.bit_sets.iter() {
            let set = self.bit_sets.entry(map_index).or_insert(S::ZERO);
            *set |= other_set;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    /// Test [`IndexSet`] index insert ops.
    #[test]
    fn test_index_set_insert() {
        let mut set = IndexSet::<u64>::default();
        let mut indices = vec![1, 4, 6, 3, 1, 100, 123, 12, 3];

        // insert some elements into the set
        for i in indices.iter().copied() {
            set.insert(i);
        }

        // check if the set contains the same elements
        // we inserted, in ascending order
        indices.sort_unstable();
        indices.dedup();

        let set_indices: Vec<_> = set.iter().collect();
        assert_eq!(indices, set_indices);

        // check that the no. of storage elements used is lower
        // than the max no. of bitsets we would otherwise need
        let storage_elements_max = indices[indices.len() - 1] / <u64 as storage::Storage>::WIDTH;
        assert!(set.bit_sets.len() <= storage_elements_max);
    }

    /// Test [`IndexSet`] index remove ops.
    #[test]
    fn test_index_set_remove() {
        let mut set = IndexSet::<u64>::default();
        let indices = [1, 4, 6, 3, 1, 100, 123, 12, 3];
        let remove = [100, 6, 100, 12, 123, 3];

        // insert some elements into the set
        for i in indices.iter().copied() {
            set.insert(i);
        }

        // remove elements from the set
        for i in remove.iter().copied() {
            set.remove(i);
        }

        let expected: HashSet<_> = {
            let indices: HashSet<_> = indices.into_iter().collect();
            let remove: HashSet<_> = remove.into_iter().collect();
            indices.difference(&remove).copied().collect()
        };
        let got: HashSet<_> = set.iter().collect();

        assert_eq!(expected, got);
    }
}
