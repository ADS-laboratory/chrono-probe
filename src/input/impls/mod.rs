pub mod distribution;

/// Implements Input for the given type using the given closure to get the size of the input.
/// Useful for not having to create a wrapper for built-in data types.
///
/// # Syntax
///
/// (`$generate_input_closure`)(`$builder`) -> `$input`, `$get_size_closure`
///
/// # Arguments
///
/// * `$generate_input_closure` - The closure that will be used to generate the input through the builder.
///     (`|usize, $builder| -> $built_in_type`)
/// * `$builder` - The type of the builder that will be used to generate the input.
/// * `$input` - The type to implement [Input](crate::input::Input) for.
/// * `$get_size_closure` - The closure that will be used to get the size of the input.
///     (`|$built_in_type| -> usize`)
///
/// # Example
///
/// ```
/// impl_input!(Vec<i32>, |v: Vec<i32>| v.len());
/// ```
#[macro_export]
macro_rules! impl_input {
    (($generate_input_closure:expr)($builder:ty) -> $input:ty, $get_size_closure:expr) => {
        /// Implementation of "Input" for $built_in_type
        impl $crate::input::Input for $input {
            type Builder<'a> = $builder;

            /// Gets the size of the input.
            fn get_size(&self) -> usize {
                $get_size_closure(self.clone())
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
            /// struct VecBuilder<'a> {
            ///     func: &'a fn(usize) -> Vec<i32>,
            /// }
            ///
            /// impl_input!( (|size: usize, builder: VecBuilder| (builder.func)(size))(VecBuilder<'a>) -> Vec<i32>, |v: Vec<i32>| v.len());
            /// let builder: VecBuilder = todo!();
            /// let input = Vec::<i32>::generate_input(10, builder);
            /// ```
            fn generate_input(size: usize, builder: Self::Builder) -> Self {
                $generate_input_closure(size, builder)
            }
        }
    };
}
