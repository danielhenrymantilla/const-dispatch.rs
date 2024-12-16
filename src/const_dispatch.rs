#[cfg(doc)]
use super::*;

/// The whole _raison d'être_ of the crate. Statically dispatch a runtime/dynamic value so as to
/// lift it to the `const` (and thus, type-level) reälm.
///
/// This only works for _limited_ `enum`erations, such as <code>[crate::primitive]::bool</code>,
/// <code>[crate::primitive]::u8</code>, or [custom simple `enum`s][macro@crate::ConstDispatch].
///
/// The types for which this dispatching work is marked by the
/// [`ConstDispatch`][trait@ConstDispatch] marker trait.
///
/// ## Usage
///
/// The "API" of this macro is described by the following pseudo-code:
///
/// ```rust
/// # r##"
/// macro const_dispatch<T: ConstDispatch>(
///     scrutinee: T,
///     const_generic_closure: impl FnOnce(<const C: T>) -> R,
/// ) -> R
/// # "##;
/// ```
///
/// ## Examples
///
/// ### [`const_dispatch!`] and [`bool`]:
///
/// ```rust
/// use ::const_dispatch::{
///     const_dispatch,
///     primitive::bool, // <- or `prelude::*`
/// };
///
/// fn inner<const VERBOSE: bool>() {
///     // ...
/// }
///
/// fn main() {
///     let verbose = ::std::env::var("VERBOSE").map_or(false, |s| s == "1");
///     const_dispatch!(verbose, |const VERBOSE: bool| {
///         inner::<VERBOSE>()
///     })
/// }
/// ```
///
/// ### Expansion
///
/// `main` in this example just above expands to:
///
/// ```rust, ignore
/// fn main() {
///     let verbose = ::std::env::var("VERBOSE").map_or(false, |s| s == "1");
///     match verbose {
///         | true => {
///             const VERBOSE: bool = true; // <- the "arg" of the "generic closure",
///             inner::<VERBOSE>() // <- the body of the "generic closure".
///         },
///         | false => {
///             const VERBOSE: bool = false; // <- the "arg" of the "generic closure",
///             inner::<VERBOSE>() // <- the body of the "generic closure".
///         },
///     }
/// }
/// ```
///
/// ### A custom `enum`
///
/// Imagine having:
///
/// ```rust
/// pub fn some_function(b: BinOp, name: &str) {
///     match b {
///         BinOp::Add => { /* some logic */ },
///         BinOp::Xor => { /* some other logic */ },
///     }
///
///     // some common logic
///
///     /* some */ loop {
///         match b {
///             BinOp::Add => { /* some more logic */ },
///             BinOp::Xor => { /* some more other logic */ },
///         }
///     }
/// }
///
/// #[derive(Debug, PartialEq, Eq)]
/// pub enum BinOp {
///     Add,
///     Xor,
/// }
/// ```
///
/// This is technically risking to be branching a bunch of times over the value of `b`.
///
/// And rewriting the logic to avoid this may prove to be challenging, or at least non-trivial.
///
/// Now, consider instead doing the following simpler transformation:
///
/// ```rust ,ignore
/// #![feature(adt_const_params)]
///
/// // 0. Use this crate!
/// use ::const_dispatch::{const_dispatch, ConstDispatch};
///
/// // 1. Make the function "a private helper", and generic over `BinOp`:
/// fn some_function_generic<const B: BinOp>(name: &str) {
///     match B {
///         BinOp::Add => { /* some logic */ },
///         BinOp::Xor => { /* some other logic */ },
///     }
///
///     // some common logic
///
///     /* some */ loop {
///         match B {
///             BinOp::Add => { /* some more logic */ },
///             BinOp::Xor => { /* some more other logic */ },
///         }
///     }
/// }
/// // This should be easy to do for the developer; and it should be trivial
/// // for the compiler to elide these branches, since `B` is now `const`.
///
/// // 2. Re-define a "`pub`lic frontend" function
/// pub fn some_function(b: BinOp, name: &str) {
///     // This works because `BinOp : ConstDispatch`
///     const_dispatch!(b, |const B: BinOp| {
///         some_function_generic::<B>(name)
///     })
/// }
///
/// // 3. Make sure `BinOp : ConstDispatch`
/// //                             vvvvvvvvvvvvv
/// #[derive(Debug, PartialEq, Eq, ConstDispatch)]
/// # #[derive(::core::marker::ConstParamTy)]
/// pub enum BinOp {
///     Add,
///     Xor,
/// }
/// ```
///
/// > Wait, `<const B: BinOp>` is not a thing in stable Rust!
///
/// True, but you get the idea. On stable rust, for simple things, using a
/// `<const IS_BINOP_ADD: bool>` generic instead is probably the simplest.
///
/// Otherwise, you could use the type-level `enum` pattern:
///
/// ```rust
/// use ::const_dispatch::{const_dispatch, ConstDispatch};
///
/// #[derive(Debug, PartialEq, Eq, ConstDispatch)]
/// pub enum BinOp {
///     Add,
///     Xor,
///     Sub,
///     Mul,
/// }
///
/// // 1. Define some "type-level" `enum` and variants
/// //    (a helper macro could make this a breeze)
/// trait IsBinOp { const VALUE: BinOp; }
///     enum Add {} impl IsBinOp for Add { const VALUE: BinOp = BinOp::Add; }
///     enum Xor {} impl IsBinOp for Xor { const VALUE: BinOp = BinOp::Xor; }
///     enum Sub {} impl IsBinOp for Sub { const VALUE: BinOp = BinOp::Sub; }
///     enum Mul {} impl IsBinOp for Mul { const VALUE: BinOp = BinOp::Mul; }
///
/// // 2. Thanks to `const_dispatch!`, dispatch to these
/// //    (using the more advanced "macro rule" API).
/// pub fn some_function(b: BinOp, name: &str) {
///     const_dispatch!(b, BinOp, |$Variant:ident| {
///         some_function_generic::<$Variant>(name)
///     })
/// }
///
/// // 3. Profit!
/// fn some_function_generic<B: IsBinOp>(name: &str) {
///     match B::VALUE {
///         BinOp::Add => { /* some logic */ },
///         BinOp::Xor => { /* some other logic */ },
///         BinOp::Mul => { /* … */ },
///         BinOp::Sub => { /* … */ },
///     }
///
///     // some common logic
///
///     /* some */ loop {
///         # break;
///         match B::VALUE {
///             BinOp::Add => { /* some logic */ },
///             BinOp::Xor => { /* some other logic */ },
///             BinOp::Mul => { /* … */ },
///             BinOp::Sub => { /* … */ },
///         }
///     }
/// }
///
/// # fn main() { some_function(BinOp::Add, ""); }
/// ```
#[macro_export]
macro_rules! const_dispatch {
    (
        $scrutinee:expr,
        |const $C:ident: $T:ident| $body:block $(,)?
    ) => ({
        // Nicer diagnostics if `$T` is not in scope (typo? Missing `use`?) or expects generic params.
        let _: $T;

        /* NOTE: the following commented out code yields an outstanding error message when `$T!` does
        not exist. And the path shadowing-or-lack-thereof trick is the usual approach to detect
        whether a given path is in scope / to have a fallback for when there is not (`default use`).
        But for some reason, when this pattern:
            - involves the macro namespace;
            - and is done in the expansion of a macro (here, `const_dispatch!`)
                —even with `call_site()` (lack of) hygiene—,
            then Rust decides to brain-fart and bail. */

        // // This is `default use $crate::ඞ::fallback_const_dispatch! as $T!`.
        // use self::$T;
        // #[allow(unused)]
        // use __better_compile_error::*;
        // mod __better_compile_error {
        //     #![allow(warnings)]
        //     pub use $crate::ඞfallback_const_dispatch as $T;
        // }
        /* poorman's version: we will still attempt to call the macro (and expose bad diagnostics), but
        at least we'll emit the nice error alongside it. */
        if false {
            $crate::ඞ::const_dispatchǃ($scrutinee, ())
        }

        $T!($scrutinee, |const $C| $body)
    });

    // more general rule exposing a "macro rule" for the output.
    (
        $scrutinee:expr, $T:ident,
        |$_:tt $Metavar:ident : $transcriber_kind:ident| $macro_output:tt
    ) => ({
        let _: $T;
        if false {
            $crate::ඞ::const_dispatchǃ($scrutinee, ())
        }
        $T!($scrutinee, ($_ $Metavar:$transcriber_kind) => $macro_output)
    });
}
