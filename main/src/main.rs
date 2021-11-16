use conversions::*;

mod never_used {
    // Including the library `party_pooper`, even if it's scoped in a private inner module,
    // causes `FooRef` to implement `CanAutoIntoo<??>` for some type, even if that type is
    // `party_pooper::Qux`, which is itself private.
    //
    // This has the effect of breaking the `main` function. Uhm...
    #[allow(unused_imports)]
    use party_pooper;
}

pub fn main() {
    let fooref: FooRef = FooRef;

    // Rust wants to call the blanket implementation of `<FooRef as Intoo<Qux>>::intoo()`,
    // even if it has unsatisfied bounds, instead of dereferencing to `Foo`.
    let bar: Bar = fooref.intoo();

    println!("{:?} {:?}", fooref, bar);
}
