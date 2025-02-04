// check-fail

#![deny(non_exhaustive_omitted_patterns)]
//~^ WARNING unknown lint: `non_exhaustive_omitted_patterns`
//~| WARNING unknown lint: `non_exhaustive_omitted_patterns`
//~| WARNING unknown lint: `non_exhaustive_omitted_patterns`
#![allow(non_exhaustive_omitted_patterns)]
//~^ WARNING unknown lint: `non_exhaustive_omitted_patterns`
//~| WARNING unknown lint: `non_exhaustive_omitted_patterns`
//~| WARNING unknown lint: `non_exhaustive_omitted_patterns`

fn main() {
    enum Foo {
        A, B, C,
    }

    #[allow(non_exhaustive_omitted_patterns)]
    //~^ WARNING unknown lint: `non_exhaustive_omitted_patterns`
    //~| WARNING unknown lint: `non_exhaustive_omitted_patterns`
    //~| WARNING unknown lint: `non_exhaustive_omitted_patterns`
    //~| WARNING unknown lint: `non_exhaustive_omitted_patterns`
    //~| WARNING unknown lint: `non_exhaustive_omitted_patterns`
    //~| WARNING unknown lint: `non_exhaustive_omitted_patterns`
    match Foo::A {
        Foo::A => {}
        Foo::B => {}
    }
    //~^^^^ ERROR non-exhaustive patterns: `Foo::C` not covered

    match Foo::A {
        Foo::A => {}
        Foo::B => {}
        #[warn(non_exhaustive_omitted_patterns)]
        _ => {}
    }
    //~^^^ WARNING unknown lint: `non_exhaustive_omitted_patterns`
    //~| WARNING unknown lint: `non_exhaustive_omitted_patterns`
    //~| WARNING unknown lint: `non_exhaustive_omitted_patterns`
}
