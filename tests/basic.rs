use ::core::ops::Not;

#[derive(PartialEq, Eq, ::const_dispatch::ConstDispatch)]
/// Mixed feelings about it.
pub enum Mixer {
    /// +
    Add,
    /// not +
    Xor,
}


fn is_add_mixer(mixer: Mixer) -> bool {
    ::const_dispatch::const_dispatch!(mixer, |const M: Mixer| {
        // Look: we got a `const` out of our runtime value!
        const IS_ADD: bool = matches!(M, Mixer::Add);
        IS_ADD
    })
}

#[test]
fn check_compilation() {
    assert!(is_add_mixer(Mixer::Add));
    assert!(is_add_mixer(Mixer::Xor).not());
}
