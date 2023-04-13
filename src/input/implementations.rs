// TODO: maybe move this file into a new crate?

/// Implementation for some built-in data types.
/// If you want to implement Input for a data types defined outside your crate you can either create a wrapper or use this macro.
#[macro_export]
macro_rules! impl_input {
    ($built_in_type:ty, $closure:expr) => {
        impl crate::input::Input for $built_in_type {
            type Builder = fn(usize) -> Self;

            fn get_size(&self) -> usize {
                $closure(self.clone())
            }

            fn generate_input(size: usize, builder: Self::Builder) -> Self {
                builder(size)
            }
        }
    };
}

impl_input!(Vec<u8>, |v: Vec<u8>| v.len());

