//! In this mod, we just implement the Bitmask trait for all the unsigned int types.

use super::Bitmask;

const INDEX_MASKS_U8: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
const INDEX_MASKS_U16: [u16; 16] =[
    1, 2, 4, 8, 16, 32, 64, 128,
    256, 512, 1024, 2048, 4096, 8192, 16384, 32768,
];
const INDEX_MASKS_U32: [u32; 32] = [
    1, 1<<1, 1<<2, 1<<3, 1<<4, 1<<5, 1<<6, 1<<7,
    1<<8, 1<<9, 1<<10, 1<<11, 1<<12, 1<<13, 1<<14, 1<<15,
    1<<16, 1<<17, 1<<18, 1<<19, 1<<20, 1<<21, 1<<22, 1<<23,
    1<<24, 1<<25, 1<<26, 1<<27, 1<<28, 1<<29, 1<<30, 1<<31,
];
const INDEX_MASKS_U64: [u64; 64] = [
    1, 1<<1, 1<<2, 1<<3, 1<<4, 1<<5, 1<<6, 1<<7,
    1<<8, 1<<9, 1<<10, 1<<11, 1<<12, 1<<13, 1<<14, 1<<15,
    1<<16, 1<<17, 1<<18, 1<<19, 1<<20, 1<<21, 1<<22, 1<<23,
    1<<24, 1<<25, 1<<26, 1<<27, 1<<28, 1<<29, 1<<30, 1<<31,
    1<<32, 1<<33, 1<<34, 1<<35, 1<<36, 1<<37, 1<<38, 1<<39,
    1<<40, 1<<41, 1<<42, 1<<43, 1<<44, 1<<45, 1<<46, 1<<47,
    1<<48, 1<<49, 1<<50, 1<<51, 1<<52, 1<<53, 1<<54, 1<<55,
    1<<56, 1<<57, 1<<58, 1<<59, 1<<60, 1<<61, 1<<62, 1<<63,
];
const INDEX_MASKS_U128: [u128; 128] = [
    1, 1<<1, 1<<2, 1<<3, 1<<4, 1<<5, 1<<6, 1<<7,
    1<<8, 1<<9, 1<<10, 1<<11, 1<<12, 1<<13, 1<<14, 1<<15,
    1<<16,  1<<17,  1<<18,  1<<19,  1<<20,  1<<21,  1<<22,  1<<23,
    1<<24,  1<<25,  1<<26,  1<<27,  1<<28,  1<<29,  1<<30,  1<<31,
    1<<32,  1<<33,  1<<34,  1<<35,  1<<36,  1<<37,  1<<38,  1<<39,
    1<<40,  1<<41,  1<<42,  1<<43,  1<<44,  1<<45,  1<<46,  1<<47,
    1<<48,  1<<49,  1<<50,  1<<51,  1<<52,  1<<53,  1<<54,  1<<55,
    1<<56,  1<<57,  1<<58,  1<<59,  1<<60,  1<<61,  1<<62,  1<<63,
    1<<64,  1<<65,  1<<66,  1<<67,  1<<68,  1<<69,  1<<70,  1<<71,
    1<<72,  1<<73,  1<<74,  1<<75,  1<<76,  1<<77,  1<<78,  1<<79,
    1<<80,  1<<81,  1<<82,  1<<83,  1<<84,  1<<85,  1<<86,  1<<87,
    1<<88,  1<<89,  1<<90,  1<<91,  1<<92,  1<<93,  1<<94,  1<<95,
    1<<96,  1<<97,  1<<98,  1<<99,  1<<100, 1<<101, 1<<102, 1<<103,
    1<<104, 1<<105, 1<<106, 1<<107, 1<<108, 1<<109, 1<<110, 1<<111,
    1<<112, 1<<113, 1<<114, 1<<115, 1<<116, 1<<117, 1<<118, 1<<119,
    1<<120, 1<<121, 1<<122, 1<<123, 1<<124, 1<<125, 1<<126, 1<<127,
];

impl Bitmask for u8 {
    const BITS: usize = 8;
    #[inline]
    fn masks() -> &'static [Self] { &INDEX_MASKS_U8 }
}

impl Bitmask for u16 {
    const BITS: usize = 16;
    #[inline]
    fn masks() -> &'static [Self] { &INDEX_MASKS_U16 }
}

impl Bitmask for u32 {
    const BITS: usize = 32;
    #[inline]
    fn masks() -> &'static [Self] { &INDEX_MASKS_U32 }
}

impl Bitmask for u64 {
    const BITS: usize = 64;
    #[inline]
    fn masks() -> &'static [Self] { &INDEX_MASKS_U64 }
}

impl Bitmask for u128 {
    const BITS: usize = 128;
    #[inline]
    fn masks() -> &'static [Self] { &INDEX_MASKS_U128 }
}

#[cfg(test)]
mod test {
    use super::*;

    // we don't want to have to write out test implementations for all five int types for each test,
    // so we define our tests as internal generic fns and run them five times each.
    // using assert_eq! requires Debug and PartialEq, so we create this trait here just to make
    // those generic contraints easier to write. Throw in Shl too since we occasionally want to
    // construct new ints that way.
    trait TestBitmask : Bitmask + std::fmt::Debug + PartialEq<Self> + std::ops::Shl<usize, Output = Self> {}
    impl TestBitmask for u8 {}
    impl TestBitmask for u16 {}
    impl TestBitmask for u32 {}
    impl TestBitmask for u64 {}
    impl TestBitmask for u128 {}

    #[test]
    fn test_mask_arrays() {
        // just make sure that our literal arrays are correct:
        fn test<T: TestBitmask>(one: T) {
            for (i, mask) in T::masks().iter().enumerate() {
                assert_eq!(one << i, *mask);
            }
        }
        test::<u8>(1);
        test::<u16>(1);
        test::<u32>(1);
        test::<u64>(1);
        test::<u128>(1);
    }
    #[test]
    fn test_set_bit() {
        fn test<T: TestBitmask>(exp: T) {
            let mut mask = T::default();
            mask.set_bit(1);
            assert_eq!(exp, mask);
        }
        test::<u8>(0b10);
        test::<u16>(0b10);
        test::<u32>(0b10);
        test::<u64>(0b10);
        test::<u128>(0b10);
    }
    #[test]
    fn test_no_bits_set() {
        fn test<T: TestBitmask>(zero: T) {
            let mask = T::no_bits_set();
            assert_eq!(zero, mask);
        }
        test::<u8>(0);
        test::<u16>(0);
        test::<u32>(0);
        test::<u64>(0);
        test::<u128>(0);
    }
    #[test]
    fn test_all_bits_set() {
        fn test<T: TestBitmask>(max: T) {
            let mask = T::all_bits_set();
            assert_eq!(max, mask);
        }
        test::<u8>(!0);
        test::<u16>(!0);
        test::<u32>(!0);
        test::<u64>(!0);
        test::<u128>(!0);
    }
    #[test]
    fn test_only_bit() {
        fn test<T: TestBitmask>(exp: T) {
            let mask = T::only_bit(7);
            assert_eq!(exp, mask);
        }
        test::<u8>(0b10000000);
        test::<u16>(0b10000000);
        test::<u32>(0b10000000);
        test::<u64>(0b10000000);
        test::<u128>(0b10000000);
    }
    #[test]
    fn test_except_bit() {
        fn test<T: TestBitmask>(exp: T) {
            let mask = T::except_bit(6);
            assert_eq!(exp, mask);
        }
        test::<u8>(0b10111111);
        test::<u16>(!(1 << 6));
        test::<u32>(!(1 << 6));
        test::<u64>(!(1 << 6));
        test::<u128>(!(1 << 6));
    }
    #[test]
    fn test_new_mask_for_bit() {
        fn test<T: TestBitmask>(false_exp: T) {
            let mask = T::new_mask_for_bit(5, true);
            assert_eq!(mask, T::all_bits_set());
            let mask = T::new_mask_for_bit(5, false);
            assert_eq!(mask, false_exp);
        }
        test::<u8>(!(1 << 5));
        test::<u16>(!(1 << 5));
        test::<u32>(!(1 << 5));
        test::<u64>(!(1 << 5));
        test::<u128>(!(1 << 5));
    }
    #[test]
    fn test_mask_for_bit() {
        fn test<T: TestBitmask>(mask: T, exp1: T, exp2: T) {
            assert_eq!(mask.mask_for_bit(1), exp1);
            assert_eq!(mask.mask_for_bit(2), exp2);
        }
        test::<u8>(0b1010, !0, !(1 << 2));
        test::<u16>(0b1010, !0, !(1 << 2));
        test::<u32>(0b1010, !0, !(1 << 2));
        test::<u64>(0b1010, !0, !(1 << 2));
        test::<u128>(0b1010, !0, !(1 << 2));
    }
    #[test]
    fn test_set_all_bits() {
        fn test<T: TestBitmask>() {
            let mut mask = T::new_mask_for_bit(4, true);
            mask.set_all_bits();
            assert_eq!(mask, !T::default());
        }
        test::<u8>();
        test::<u16>();
        test::<u32>();
        test::<u64>();
        test::<u128>();
    }
    #[test]
    fn test_unset_all_bits() {
        fn test<T: TestBitmask>() {
            let mut mask = T::new_mask_for_bit(3, true);
            mask.unset_all_bits();
            assert_eq!(mask, T::default());
        }
        test::<u8>();
        test::<u16>();
        test::<u32>();
        test::<u64>();
        test::<u128>();
    }
    #[test]
    fn test_get_bit() {
        fn test<T: TestBitmask>(i: usize) {
            let mut mask = T::default();
            assert!(!mask.get_bit(i));
            mask.set_bit(i);
            assert!(mask.get_bit(i));
        }
        // can finally use some bigger idxs here:
        test::<u8>(7);
        test::<u16>(14);
        test::<u32>(23);
        test::<u64>(55);
        test::<u128>(99);
    }
    #[test]
    fn test_unset_bit() {
        fn test<T: TestBitmask>(i: usize) {
            let mut mask = T::all_bits_set();
            assert!(mask.get_bit(i));
            mask.unset_bit(i);
            assert!(!mask.get_bit(i));
        }
        test::<u8>(3);
        test::<u16>(10);
        test::<u32>(20);
        test::<u64>(40);
        test::<u128>(70);
    }
    #[test]
    fn test_set_bit_if() {
        fn test<T: TestBitmask>(i: usize) {
            let mut mask = T::all_bits_set();
            assert!(mask.get_bit(i));
            mask.set_bit_if(i, true);
            assert!(mask.get_bit(i));
            mask.set_bit_if(i, false);
            assert!(!mask.get_bit(i));
        }
        test::<u8>(6);
        test::<u16>(11);
        test::<u32>(21);
        test::<u64>(45);
        test::<u128>(79);
    }
    #[test]
    fn test_switch_bit() {
        fn test<T: TestBitmask>(i: usize) {
            let mut mask = T::all_bits_set();
            assert!(mask.get_bit(i));
            mask.switch_bit(i);
            assert!(!mask.get_bit(i));
            mask.switch_bit(i);
            assert!(mask.get_bit(i));
        }
        test::<u8>(6);
        test::<u16>(11);
        test::<u32>(21);
        test::<u64>(45);
        test::<u128>(79);
    }
    #[test]
    fn test_or_eq() {
        fn test<T: TestBitmask>(mut a: T, b: T, exp: T) {
            a.or_eq(b);
            assert_eq!(a, exp);
        }
        test::<u8>(0b1010, 0b0100, 0b1110);
        test::<u16>(0b1010, 0b0100, 0b1110);
        test::<u32>(0b1010, 0b0100, 0b1110);
        test::<u64>(0b1010, 0b0100, 0b1110);
        test::<u128>(0b1010, 0b0100, 0b1110);
    }
    #[test]
    fn test_filter() {
        fn test<T: TestBitmask>(mut a: T, b: T, exp: T) {
            a.filter(b);
            assert_eq!(a, exp);
        }
        test::<u8>(0b1010, 0b1100, 0b0010);
        test::<u16>(0b1010, 0b1100, 0b0010);
        test::<u32>(0b1010, 0b1100, 0b0010);
        test::<u64>(0b1010, 0b1100, 0b0010);
        test::<u128>(0b1010, 0b1100, 0b0010);
    }
}
