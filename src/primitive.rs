//! The "`impl`s" for primitive types such as `bool` and `u8`
//!
//! ```rust, ignore
//! /// Pseudo-code
//! mod ::const_dispatch::primitive {
//!     impl ConstDispatch for bool { /* magic */ }
//!     impl ConstDispatch for u8 { /* magic */ }
//! }
//! ```
//!
//! ## Example
//!
//! ```rust
//! use ::const_dispatch::{const_dispatch, primitive::bool};
//!
//! fn inner<const VERBOSE: bool>() {
//!     // ...
//! }
//!
//! fn main() {
//!     let verbose = ::std::env::var("VERBOSE").map_or(false, |s| s == "1");
//!     const_dispatch!(verbose, |const VERBOSE: bool| {
//!         inner::<VERBOSE>()
//!     })
//! }
//! ```
//!
//! Make sure to import these (either individually, or as a blob import from the
//! <code>[crate::prelude]::*</code>) in order for [`const_dispatch!`][crate::const_dispatch] to
//! work with `u8` and `bool` properly.

#[doc(hidden)]
#[macro_export]
macro_rules! bool {
    (
        $scrutinee:expr, |const $C:ident| $body:block
    ) => (
        match $scrutinee {
            | true => {
                const $C: ::core::primitive::bool = true;
                {
                    $crate::ඞ::try_hide!($scrutinee);
                    $body
                }
            },
            | false => {
                const $C: ::core::primitive::bool = false;
                {
                    $crate::ඞ::try_hide!($scrutinee);
                    $body
                }
            },
        }
    );

    (
        $scrutinee:expr,
        $macro_input:tt => $macro_output:tt
    ) => ({
        macro_rules! __emit__ { $macro_input => $macro_output }
        match $scrutinee {
            | true => {
                $crate::ඞ::try_hide!($scrutinee);
                emit! {
                    true
                }
            },
            | false => {
                $crate::ඞ::try_hide!($scrutinee);
                emit! {
                    false
                }
            },
        }
    });
}

#[doc(no_inline)]
pub use bool;

impl crate::ඞ::MacroDerived for bool {} /* as bool */

::const_dispatch_proc_macros::ඞimpl_for_u8!();

#[doc(no_inline)]
pub use u8;

impl crate::ඞ::MacroDerived for u8 {} /* as u8 */
