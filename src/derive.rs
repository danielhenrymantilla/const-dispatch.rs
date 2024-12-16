#[doc(hidden)] /** Not part of the public API (for now) */ #[macro_export]
// #[cfg_attr(feature = "better-docs", doc(cfg(not(proc_macros))))]
// /// When the (default) `"proc-macros"` feature is disabled, use this instead of
// /// <code>#\[[derive]\([ConstDispatch][macro@ConstDispatch]\)\]</code>.
// ///
// /// See the documentation of <code>#\[derive\([ConstDispatch][macro@ConstDispatch]\)\]</code> for
// /// more info.
// #[macro_export]
// macro_rules! derive_ConstDispatch {
//     (
//         $($simple_enum_definition:tt)*
//     ) => (
//         $crate::ඞ::derive_ConstDispatch! {
//             $($simple_enum_definition)*
//         }
//     );
// }

// #[doc(hidden)] /** Not part of the public API */ #[macro_export]
macro_rules! ඞderive_ConstDispatch {(
    $(#$attr:tt)*
    $(pub $(@$if_pub:tt)? $(($($vis:tt)*))?)?
    enum $EnumName:ident {
        $(
            $(#$var_attr:tt)*
            $Variant:ident $(= $disc:expr)?
        ),* $(,)?
    }
) => (
    $(#$attr)*
    $(pub $(($($vis)*))?)?
    enum $EnumName {
        $(
            $(#$var_attr)*
            $Variant $(= $disc)?,
        )*
    }

    $crate::ඞ::paste! {
        #[doc = "`const_dispatch!` helper for [`" $EnumName "`]"]
        #[allow(nonstandard_style)]
        #[doc(hidden)]
        $($($if_pub)?
            #[macro_export]
        )?
        macro_rules! [< ඞ $EnumName >] {
            (
                $scrutinee:expr,
                |const $C:ident| $body:block
            ) => (
                match $scrutinee {
                    $(
                        | $EnumName::$Variant => {
                            const $C: $EnumName = $EnumName::$Variant;
                            {
                                $crate::ඞ::try_hide!($scrutinee);
                                $body
                            }
                        },
                    )*
                }
            );

            (
                $scrutinee:expr,
                $macro_input:tt => $macro_output:tt
            ) => ({
                macro_rules! __emit__ { $macro_input => $macro_output }
                match $scrutinee {
                    $(
                        | $EnumName::$Variant => {
                            $crate::ඞ::try_hide!($scrutinee);
                            __emit__! { $Variant }
                        },
                    )*
                }
            });
        }

        #[doc(inline)]
        $(pub $(($($vis)*))?)? use [< ඞ $EnumName >] as $EnumName;

        impl $crate::ඞ::MacroDerived for $EnumName {}
    }
)}
