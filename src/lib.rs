//! # Final_fn
//! This crate provides ***final_fn*** macros, what executes given code when leaving code block
//! # Example
//! ```
//! use final_fn::final_fn;
//!
//! fn main() {
//!     final_fn!({
//!         println!("End of main!");
//!     });
//!
//!     println!("Hello world!");
//! }
//! ```

#![no_std]

/// # How this macros works
/// This macros creates ***FinalFunc*** structure, what implements Drop trait
/// # Examples
/// This short examples show, how to use this macros!
/// - **Code block**
/// ```
/// use final_fn::final_fn;
///
/// fn main() {
///     final_fn!({
///         println!("End of main!");
///     });
///
///     println!("Hello world!");
/// }
/// ```
/// - **Expression**
/// ```
/// use final_fn::final_fn;
///
/// fn main() {
///     final_fn!(
///         println!("End of main!")
///     );
///
///     println!("Hello world!");
/// }
/// ```
#[macro_export]
macro_rules! final_fn {
    ($code:block) => {
        let r#final: final_fn::FinalFunc<_> = (|| $code).into();
    };
    ($code:expr) => {
        let r#final: final_fn::FinalFunc<_> = (|| $code).into();
    };
}



pub struct FinalFunc<T>
where
    T: Fn(),
{
    func: T,
}

impl<T> Drop for FinalFunc<T>
where
    T: Fn(),
{
    fn drop(&mut self) {
        (self.func)()
    }
}

impl<T> From<T> for FinalFunc<T>
where
    T: Fn(),
{
    fn from(value: T) -> Self {
        Self { func: value }
    }
}
