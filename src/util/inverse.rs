use crate::F;

// TODO make generic?
/// A value that caches its additive inverse.
#[derive(Debug, Default)]
pub struct AddInv {
    pub base: F,
    pub inv: F,
}

impl AddInv {
    pub fn new(base: F) -> Self {
        crate::check_float_01!(base);
        Self { base, inv: 1.0 - base }
    }

    pub fn set(&mut self, base: F) {
        crate::check_float_01!(base);
        self.base = base;
        self.inv = 1.0 - base;
    }

    pub fn get_base(&self) -> F {
        self.base
    }

    pub fn get_inv(&self) -> F {
        self.inv
    }
}

/// A value that caches its multiplicative inverse.
#[derive(Debug, Default)]
pub struct MulInv {
    base: F,
    inv: F,
}

impl MulInv {
    pub fn set(&mut self, base: F) {
        crate::check_float_nonzero!(base);
        self.base = base;
        self.inv = 1.0 / base;
    }

    pub fn get_base(&self) -> F {
        self.base
    }

    pub fn get_inv(&self) -> F {
        self.inv
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_add_inv() {
        let mut val = AddInv::new(0.4);
        assert_eq!(val.base, 0.4);
        assert_eq!(val.inv, 0.6);
        assert_eq!(val.get_base(), 0.4);
        assert_eq!(val.get_inv(), 0.6);

        val.set(0.3);
        assert_eq!(val.base, 0.3);
        assert_eq!(val.inv, 0.7);
        assert_eq!(val.get_base(), 0.3);
        assert_eq!(val.get_inv(), 0.7);
    }
    #[test]
    fn test_mul_inv() {
        let mut val = MulInv::default();
        assert_eq!(val.get_base(), 0.0);
        assert_eq!(val.get_inv(), 0.0);

        val.set(2.0);
        assert_eq!(val.get_base(), 2.0);
        assert_eq!(val.get_inv(), 0.5);

        val.set(0.2);
        assert_eq!(val.get_base(), 0.2);
        assert_eq!(val.get_inv(), 1.0 / 0.2);
    }
}
