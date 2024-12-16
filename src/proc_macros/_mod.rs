//! Crate not intended for direct use.
//! Use https:://docs.rs/const-dispatch instead.
// Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template
#![allow(nonstandard_style, unused_imports)]

use ::core::{
    mem,
    ops::Not as _,
};
use ::proc_macro::*;

macro_rules! tts {
    [
        $($tt:expr),* $(,)?
    ] => (
        [
            $(
                TokenTree::from($tt),
            )*
        ]
        .into_iter()
    );
}

#[proc_macro_attribute]
#[doc(hidden)] /** Not part of the public API */ pub
fn ඞexterminate(
    args: TokenStream,
    _input: TokenStream,
) -> TokenStream
{
    assert!(args.is_empty());
    TokenStream::new()
}

#[proc_macro_derive(ConstDispatch)] pub
fn __(
    input: TokenStream
) -> TokenStream
{
    let to_be_braced = Iterator::chain(
        tts![
            Punct::new('#', proc_macro::Spacing::Alone),
            Group::new(
                Delimiter::Bracket,
                tts![
                    Punct::new(':', proc_macro::Spacing::Joint),
                    Punct::new(':', proc_macro::Spacing::Alone),
                    Ident::new("const_dispatch", Span::mixed_site()),
                    Punct::new(':', proc_macro::Spacing::Joint),
                    Punct::new(':', proc_macro::Spacing::Alone),
                    Ident::new("ඞ", Span::mixed_site()),
                    Punct::new(':', proc_macro::Spacing::Joint),
                    Punct::new(':', proc_macro::Spacing::Alone),
                    Ident::new("exterminate", Span::mixed_site()),
                ]
                .collect()
            )
        ],
        input,
    );
    tts![
        Punct::new(':', proc_macro::Spacing::Joint),
        Punct::new(':', proc_macro::Spacing::Alone),
        Ident::new("const_dispatch", Span::mixed_site()),
        Punct::new(':', proc_macro::Spacing::Joint),
        Punct::new(':', proc_macro::Spacing::Alone),
        Ident::new("ඞ", Span::mixed_site()),
        Punct::new(':', proc_macro::Spacing::Joint),
        Punct::new(':', proc_macro::Spacing::Alone),
        Ident::new("derive_ConstDispatch", Span::mixed_site()),
        Punct::new('!', proc_macro::Spacing::Alone),
        Group::new(Delimiter::Brace, to_be_braced.collect()),
    ].collect()
}

#[proc_macro]
#[doc(hidden)] /** Not part of the public API */ pub
fn ඞimpl_for_u8(
    _input: TokenStream,
) -> TokenStream
{
    // One-off helper macro.
    // Hygiene plays no role in the output, so we simplify the `syn`-less impl by using strings.
    let branches_const =
        (0..=u8::MAX)
            .map(|n| format!(r#"
                {n}_u8 => {{
                    const $C: ::core::primitive::u8 = $n;
                    $body
                }},
            "#))
            .collect::<String>()
    ;
    let branches_macro =
        (0..=u8::MAX)
            .map(|n| format!(r#"
                {n}_u8 => {{
                    __emit__! {{ $n }}
                }},
            "#))
            .collect::<String>()
    ;
    format!(r#"
        #[doc(hidden)]
        #[macro_export]
        macro_rules! u8 {{
            (
                $scrutinee:expr,
                |const $C:ident| $body:block
            ) => (
                match $scrutinee {{
                    {branches_const}
                }}
            );

            (
                $scrutinee:expr,
                $macro_input:tt => $macro_output:tt
            ) => ({{
                macro_rules! __emit__ {{ $macro_input => $macro_output }}
                match $scrutinee {{
                    {branches_macro}
                }}
            }});
        }}
    "#).parse().unwrap()
}
