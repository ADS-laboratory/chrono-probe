pub mod distribution;

/// Implements Input for the given type using the given closure to get the size of the input.
/// Useful for not having to create a wrapper for built-in data types.
/// 
/// # Arguments
/// 
/// * `$built_in_type` - The type to implement [Input](crate::input::Input) for.
/// * `$closure` - The closure that will be used to get the size of the input.
/// 
/// # Example
/// 
/// ```
/// impl_input!(Vec<i32>, |v: Vec<i32>| v.len());
/// ```
#[macro_export]
macro_rules! impl_input {
    ($built_in_type:ty, $closure:expr) => {
        /// Implementation of "Input" for $built_in_type
        impl crate::input::Input for $built_in_type {
            type Builder = fn(usize) -> Self;

            /// Gets the size of the input.
            fn get_size(&self) -> usize {
                $closure(self.clone())
            }
            
            /// Generates an input of the given size using the given builder.
            /// 
            /// # Arguments
            /// 
            /// * `size` - The size of the input to be generated.
            /// * `builder` - The builder that will be used to generate the input (it's basically a function).
            /// 
            /// # Example
            /// 
            /// ```
            /// use time_complexity_plot::input::Input;
            /// 
            /// fn builder(size: usize) -> Vec<i32> {
            ///    vec![0; size]
            /// }
            /// 
            /// impl_input!(Vec<i32>, |v: Vec<i32>| v.len());
            /// let input = Vec::<i32>::generate_input(10, builder);
            /// ```
            fn generate_input(size: usize, builder: Self::Builder) -> Self {
                builder(size)
            }
        }
    };
}