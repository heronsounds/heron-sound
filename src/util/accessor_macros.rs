//! Use these macros to avoid boilerplate when writing dsp components, particularly specs.

/// helper to create get_val() and get_val_mut() fns.
#[macro_export]
macro_rules! getters {
    ($field:ident, $fn:ident, $fn_mut:ident, $ty:ty) => {
        pub fn $fn(&self) -> &$ty {
            &self.$field
        }
        pub fn $fn_mut(&mut self) -> &mut $ty {
            &mut self.$field
        }
    };
}

/// helper to create get_val() and set_val() fns.
///
/// Where [getters] is useful for structs and other non-Copy types,
/// this macro is more useful for primitives and Copy types.
#[macro_export]
macro_rules! accessors {
    ($field:ident, $fn_get:ident, $fn_set:ident, $ty:ty) => {
        pub fn $fn_get(&self) -> $ty {
            self.$field
        }
        pub fn $fn_set(&mut self, val: $ty) {
            self.$field = val;
        }
    };
}

/// helper to create get_val() and get_val_mut() fns that take indexes.
#[macro_export]
macro_rules! index_getters {
    ($field:ident, $fn_get:ident, $fn_mut:ident, $ty:ty) => {
        pub fn $fn_get(&self, idx: usize) -> &$ty {
            &self.$field[idx]
        }
        pub fn $fn_mut(&mut self, idx: usize) -> &mut $ty {
            &mut self.$field[idx]
        }
    };
}

/// helper to create get_val() and set_val() fns that take indexes.
#[macro_export]
macro_rules! index_accessors {
    ($field:ident, $fn_get:ident, $fn_set:ident, $ty:ty) => {
        pub fn $fn_get(&self, idx: usize) -> $ty {
            self.$field[idx]
        }
        pub fn $fn_set(&mut self, idx: usize, val: $ty) {
            self.$field[idx] = val;
        }
    };
}
