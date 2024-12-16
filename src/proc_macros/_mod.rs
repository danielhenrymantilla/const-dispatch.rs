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
        .collect::<TokenStream>()
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
            Punct::new('#', Spacing::Alone),
            Group::new(
                Delimiter::Bracket,
                tts![
                    Punct::new(':', Spacing::Joint),
                    Punct::new(':', Spacing::Alone),
                    Ident::new("const_dispatch", Span::mixed_site()),
                    Punct::new(':', Spacing::Joint),
                    Punct::new(':', Spacing::Alone),
                    Ident::new("ඞ", Span::mixed_site()),
                    Punct::new(':', Spacing::Joint),
                    Punct::new(':', Spacing::Alone),
                    Ident::new("exterminate", Span::mixed_site()),
                ]
            )
        ]
            .into_iter()
        ,
        input,
    );
    tts![
        Punct::new(':', Spacing::Joint),
        Punct::new(':', Spacing::Alone),
        Ident::new("const_dispatch", Span::mixed_site()),
        Punct::new(':', Spacing::Joint),
        Punct::new(':', Spacing::Alone),
        Ident::new("ඞ", Span::mixed_site()),
        Punct::new(':', Spacing::Joint),
        Punct::new(':', Spacing::Alone),
        Ident::new("derive_ConstDispatch", Span::mixed_site()),
        Punct::new('!', Spacing::Alone),
        Group::new(Delimiter::Brace, to_be_braced.collect()),
    ]
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
                    const $C: ::core::primitive::u8 = {n};
                    {{
                        $crate::ඞ::try_hide!($scrutinee);
                        $body
                    }}
                }},
            "#))
            .collect::<String>()
    ;
    let branches_macro =
        (0..=u8::MAX)
            .map(|n| format!(r#"
                {n}_u8 => {{
                    $crate::ඞ::try_hide!($scrutinee);
                    __emit__! {{ {n} }}
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

/// If `input = <identifier ≠ self>`, it emits a `let <identifier>: !;` to prevent the
/// `const`-dispatched code from accidently using the runtime value rather than the `const`.
#[proc_macro]
#[doc(hidden)] /** Not part of the public API */ pub
fn ඞtry_hide(
    input: TokenStream,
) -> TokenStream
{
    let ident = match input.into_iter().next().unwrap() {
        TokenTree::Group(g) if g.delimiter() == Delimiter::None => {
            let mut inner_tts = g.stream().into_iter();
            match (inner_tts.next(), inner_tts.next()) {
                (Some(TokenTree::Ident(ident)), None) if ident.to_string() != "self" => ident,
                _ => return <_>::default(),
            }
        },
        // currently not a reachable code path off a `$scrutinee:expr` arg, but who knows.
        TokenTree::Ident(ident) if ident.to_string() != "self" => ident,
        _ => return <_>::default(),
    };
    let allow_unused = Group::new(
        Delimiter::Bracket,
        tts![
            Ident::new("allow", Span::mixed_site()),
            Group::new(
                Delimiter::Parenthesis,
                tts![
                    Ident::new("unused", Span::mixed_site()),
                ],
            ),
        ],
    );
    tts![
        Punct::new('#', Spacing::Alone), allow_unused.clone(),
        // fn #ident() {}
        Ident::new("fn", Span::mixed_site()),
        ident.clone(),
        Group::new(Delimiter::Parenthesis, <_>::default()),
        Group::new(Delimiter::Brace, <_>::default()),

        Punct::new('#', Spacing::Alone), allow_unused,
        // let #ident: ::const_dispatch::ඞ::Never;
        Ident::new("let", Span::mixed_site()),
        ident,
        Punct::new(':', Spacing::Alone),

        Punct::new(':', Spacing::Joint),
        Punct::new(':', Spacing::Alone),
        Ident::new("const_dispatch", Span::mixed_site()),
        Punct::new(':', Spacing::Joint),
        Punct::new(':', Spacing::Alone),
        Ident::new("ඞ", Span::mixed_site()),
        Punct::new(':', Spacing::Joint),
        Punct::new(':', Spacing::Alone),
        Ident::new("Never", Span::mixed_site()),

        Punct::new(';', Spacing::Alone),
    ]
}
