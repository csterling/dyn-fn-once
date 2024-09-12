//! Provides dynamically-typed self-consuming closures, [DynFnOnce] and [DynFnOnceSend].
//! 
//! E.g.
//! ```
//! use std::fmt::Write;
//! use dyn_fn_once::DynFnOnce;
//!
//! /// Creates a closure which either multiplies or adds two numbers, and stores the result
//! /// in `target`, returning the given string with the result appended.
//! fn create_binary_operation<'capture>(
//!     target: &'capture mut u32,
//!     mut units: String, 
//!     mul: bool
//! ) -> DynFnOnce<'capture, (u32, u32), String> {
//!     if mul {
//!         let statically_typed_closure = move |a, b| {
//!             let product = a * b;
//!             units.write_fmt(format_args!(" = {product}")).unwrap();
//!             *target = product;
//!             units
//!         };
//!         
//!         // N.B. the closure takes multiple arguments, so we have to turn them
//!         // into a single tuple-typed argument
//!         DynFnOnce::from(|(a, b)| statically_typed_closure(a, b))
//!     } else {
//!         let statically_typed_closure = move |a, b| {
//!             let sum = a + b;
//!             units.write_fmt(format_args!(" = {sum}")).unwrap();
//!             *target = sum;
//!             units
//!         };
//!         
//!         // N.B. the closure takes multiple arguments, so we have to turn them
//!         // into a single tuple-typed argument
//!         DynFnOnce::from(|(a, b)| statically_typed_closure(a, b))
//!     }
//! }
//!
//! let mut target: u32 = 0;
//!
//! assert_eq!(
//!     create_binary_operation(&mut target, String::from("Hertz"), true).call((3, 4)),
//!     String::from("Hertz = 12")
//! );
//! assert_eq!(target, 12);
//!
//! assert_eq!(
//!     create_binary_operation(&mut target, String::from("Watts"), false).call((3, 4)),
//!     String::from("Watts = 7")
//! );
//! assert_eq!(target, 7);
//!
//! ```
mod dyn_fn_once;
pub use dyn_fn_once::DynFnOnce;

mod dyn_fn_once_send;
pub use dyn_fn_once_send::DynFnOnceSend;
