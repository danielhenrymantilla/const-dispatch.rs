#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![allow(uncommon_codepoints)] // `fn const_dispatchǃ` below
#![warn(missing_docs)]
#![cfg_attr(feature = "better-docs",
    feature(doc_cfg),
)]

pub mod prelude {
    //! The crate's prelude, designed to be `*`-blob imported.
    #[doc(no_inline)]
    pub use crate::{
        primitive::*,
        const_dispatch,
        ConstDispatch,
    };
}

pub mod primitive;

/// Derives/`impl`ements [`ConstDispatch`][trait@ConstDispatch] for the given type.
///
/// The input must be a simple `enum`: that is, an `enum` whose variants have no payloads
/// (often also called a "C `enum`").
#[doc(inline)] // TODO
pub use ::const_dispatch_proc_macros::ConstDispatch;

/// Marker trait for types which may be fed to [`const_dispatch!`].
///
/// It is not to be implemented manually: only through its eponymous
/// <code>#\[[derive]\([ConstDispatch][macro@ConstDispatch]\)\]</code>.
///
/// [derive]: [macro@derive]
#[diagnostic::on_unimplemented(
    note = "the `enum` definition is missing a `#[derive(ConstDispatch)]`",
)]
pub trait ConstDispatch : ඞ::MacroDerived {}

mod const_dispatch_macro;

mod derive;

// macro internals
#[doc(hidden)] /** Not part of the public API */ pub
mod ඞ {
    #![allow(missing_docs)]
    pub use ::core;

    pub use crate::ඞderive_ConstDispatch as derive_ConstDispatch;

    pub use ::const_dispatch_proc_macros::ඞexterminate as exterminate;
    pub use ::const_dispatch_proc_macros::ඞtry_hide as try_hide;

    pub use ::paste::paste;
    pub use ::never_say_never::Never;

    pub trait MacroDerived {}
    impl<T : ?Sized> ConstDispatch for T where Self : MacroDerived {}

    /* See the NOTE: above as to why we are not using this anymore. */
    // #[doc(hidden)] /** Not part of the public API */ #[macro_export]
    // macro_rules! ඞfallback_const_dispatch {(
    //     $scrutinee:expr, $($rest:tt)*
    // ) => (
    //     $crate::ඞ::const_dispatchǃ($scrutinee, ())
    // )}
    // #[doc(inline)]
    // pub use ඞfallback_const_dispatch as fallback_const_dispatch;

    use Sized as FnOnceConst;
    use crate::ConstDispatch;

    pub fn
    /* macro */ const_dispatchǃ(_scrutinee: impl ConstDispatch, _dispatch: impl FnOnceConst) -> ! {
        unimplemented!()
    }
}

#[doc = include_str!("compile_fail_tests.md")]
mod _compile_fail_tests {}
