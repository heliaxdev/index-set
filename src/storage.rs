//! Storage integers for a [`super::IndexSet`].

use core::hash::Hash;
use core::ops::{BitAnd, BitAndAssign, BitOrAssign, Not};

macro_rules! impl_storage_for {
    ($primitive:ty) => {
        impl $crate::storage::Storage for $primitive {
            const ZERO: $primitive = 0;

            #[inline(always)]
            fn from_usize(x: usize) -> $primitive {
                x as $primitive
            }

            #[inline(always)]
            fn num_of_high_bits(self) -> usize {
                self.count_ones() as usize
            }
        }
    };
}

impl_storage_for!(u8);
impl_storage_for!(u16);
impl_storage_for!(u32);
impl_storage_for!(u64);
impl_storage_for!(u128);

/// The storage unit for the bits in a [`super::IndexSet`].
///
/// Any primitive unsigned integer type will do.
pub trait Storage:
    Sized
    + Hash
    + PartialOrd
    + Ord
    + PartialEq
    + Eq
    + Copy
    + PartialEq<Self>
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOrAssign
    + Not<Output = Self>
{
    /// The value 0 of this [`Storage`] integer type.
    const ZERO: Self;

    /// The width, in bytes, of this [`Storage`] integer type.
    const WIDTH: usize = ::core::mem::size_of::<Self>();

    /// Convert a [`usize`] to a value of [`Self`].
    fn from_usize(x: usize) -> Self;

    /// Count the number of bits set in [`Self`].
    fn num_of_high_bits(self) -> usize;
}
