//! Set data structures optimized to store [`usize`] values.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod btree;
mod macros;
mod storage;
pub mod vec;

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
