use ::const_dispatch::prelude::*;

fn demo(n: u8) {
    ::const_dispatch::const_dispatch!(n, |const N: u8| {
        dbg!(N);
    })
}

fn demo_macro(n: u8) {
    ::const_dispatch::const_dispatch!(n, u8, |$N:literal| {
        let _: [(); $N];
    })
}

#[test]
fn check_compilation() {
    demo(0);
    demo_macro(0);
}
