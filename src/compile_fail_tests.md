# The following snippets fail to compile:

## Using a type which does not implement `ConstDispatch`

```rust ,compile_fail
use ::const_dispatch::*;

#[derive(PartialEq, Eq, /* no ConstDispatch */)]
/// Mixed feelings about it.
pub enum Mixer {
    /// +
    Add,
    /// not +
    Xor,
}

fn demo(mixer: Mixer) {
    ::const_dispatch::const_dispatch!(mixer, |const M: Mixer| {
        const IS_ADD: bool = matches!(M, Mixer::Add);
        dbg!(IS_ADD);
    });
}
```

## Typ(e-)o

```rust ,compile_fail
use ::const_dispatch::*;

#[derive(PartialEq, Eq, /* no ConstDispatch */)]
/// Mixed feelings about it.
pub enum Mixer {
    /// +
    Add,
    /// not +
    Xor,
}

fn demo(mixer: Mixer) {
    ::const_dispatch::const_dispatch!(mixer, |const M: Mixed| {
        const IS_ADD: bool = matches!(M, Mixer::Add);
        dbg!(IS_ADD);
    });
}
```

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->
