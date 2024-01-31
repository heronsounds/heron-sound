use std::{ops, cmp};

/// Trait for types that can be used as the underlying type of a `Bitmask`.
///
/// Implemented for all unsigned int types.
/// This may want to move to a separate crate at some point.
pub trait Bitmask : Sized
    + 'static
    + Copy
    + Default
    + cmp::PartialEq
    + ops::Shr<usize, Output = Self>
    + ops::Not<Output = Self>
    + ops::BitOrAssign<Self>
    + ops::BitAndAssign<Self>
    + ops::BitXorAssign<Self>
    + ops::BitAnd<Self, Output = Self>
    + ops::BitXor<Self, Output = Self>
{
    /// Number of bits contained in this type (use this when bounds-checking)
    const BITS: usize;

    /// Ref to array of masks, each containing exactly one set bit.
    fn masks() -> &'static [Self];

    /// create a new mask with no bits set.
    #[inline]
    fn no_bits_set() -> Self {
        Self::default()
    }

    /// create a new mask with all bits set.
    #[inline]
    fn all_bits_set() -> Self {
        !Self::default()
    }

    /// create a new mask with only the i'th bit set.
    #[inline]
    fn only_bit(i: usize) -> Self {
        crate::check_int_less_than!(i, Self::BITS);
        Self::masks()[i]
    }

    /// create a new mask with all bits set except for the i'th bit.
    #[inline]
    fn except_bit(i: usize) -> Self {
        crate::check_int_less_than!(i, Self::BITS);
        !Self::masks()[i]
    }

    /// create a new mask with all bits set if val is true,
    /// or with all bits except the i'th bit if val is false.
    #[inline]
    fn new_mask_for_bit(i: usize, val: bool) -> Self {
        if val { Self::all_bits_set() } else { Self::except_bit(i) }
    }

    /// create a new mask that can be used to sync the i'th bit of another bitmask to this one.
    ///
    /// what this means: if i'th bit is set, returns mask with all bits set.
    /// otherwise, returns mask with all bits except i'th bit set.
    #[inline]
    fn mask_for_bit(&self, i: usize) -> Self {
        crate::check_int_less_than!(i, Self::BITS);
        let mask = Self::masks()[i];
        (*self & mask) ^ !mask
    }

    /// set all bits to true
    #[inline]
    fn set_all_bits(&mut self) {
        *self = !Self::default();
    }

    /// set all bits to false
    #[inline]
    fn unset_all_bits(&mut self) {
        *self = Self::default();
    }

    /// return true if the i'th bit is set
    #[inline]
    fn get_bit(&self, i: usize) -> bool {
        let one = Self::masks()[0];
        (*self >> i) & one == one
    }

    /// set the i'th bit to true
    #[inline]
    fn set_bit(&mut self, i: usize) {
        crate::check_int_less_than!(i, Self::BITS);
        *self |= Self::masks()[i]
    }

    /// set the i'th bit to false
    #[inline]
    fn unset_bit(&mut self, i: usize) {
        crate::check_int_less_than!(i, Self::BITS);
        *self &= !Self::masks()[i]
    }

    /// set or unset i'th bit according to val.
    #[inline]
    fn set_bit_if(&mut self, i: usize, val: bool) {
        if val { self.set_bit(i) } else { self.unset_bit(i) }
    }

    /// switch i'th bit from false to true, or true to false
    #[inline]
    fn switch_bit(&mut self, i: usize) {
        crate::check_int_less_than!(i, Self::BITS);
        *self ^= Self::masks()[i];
    }

    /// or_assign this bitset with other
    #[inline]
    fn or_eq(&mut self, other: Self) {
        *self |= other;
    }

    /// unset all bits that are set in other
    #[inline]
    fn filter(&mut self, other: Self) {
        *self &= !other;
    }
}
