//! Passive conversion [`Froom`] and active conversion [`Intoo`].
//!
//! The active conversion has a [blanket implementation](Intoo#impl-Intoo<U>)
//! if the passive conversion is defined and the [`CanAutoIntoo`] flag is enabled.
//!
//! Three types [`Foo`], [`FooRef`] and [`Bar`] are provided to play with.

/// Active conversion from `T` to `U`.
pub trait Intoo<U> {
    fn intoo(&self) -> U;
}

/// Passive conversion from `T` to `U`.
pub trait Froom<T> {
    fn froom(_: &T) -> Self;
}

/// Flag trait to enable the blanket implementation of [`Intoo<U>`].
pub trait CanAutoIntoo<U> {}

/// Blanket implementation.
///
/// Automatically implements [`Intoo`] if the corresponding [`Froom`] is available.
/// Requires [`CanAutoIntoo`] in order to be enabled.
impl<T, U> Intoo<U> for T
where
    U: Froom<T>,
    T: CanAutoIntoo<U>,
{
    fn intoo(&self) -> U {
        U::froom(self)
    }
}

/// Can be converted to [`Bar`].
#[derive(Debug)]
pub struct Foo;

/// Can be converted from [`Foo`].
#[derive(Debug)]
pub struct Bar;

/// Can dereference to [`Foo`].
#[derive(Debug)]
pub struct FooRef;

impl std::ops::Deref for FooRef {
    type Target = Foo;
    fn deref(&self) -> &Foo {
        &Foo
    }
}

// conversions from `Foo` to `Bar`
impl Froom<Foo> for Bar {
    fn froom(_: &Foo) -> Bar {
        Bar
    }
}

// enable the blanket implementation `Foo: Intoo<Bar>`
impl CanAutoIntoo<Bar> for Foo {}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::*;

    #[test]
    fn foo_bar_work() {
        assert_impl_all!(Foo: Intoo<Bar>, CanAutoIntoo<Bar>);
        assert_impl_all!(Bar: Froom<Foo>);
    }

    #[test]
    fn fooref_works() {
        assert_impl_all!(FooRef: std::ops::Deref<Target = Foo>);
        assert_not_impl_any!(FooRef: Intoo<Bar>, CanAutoIntoo<Bar>);
        assert_not_impl_any!(Bar: Froom<FooRef>);
    }

    #[test]
    fn blanket_impl_works() {
        struct T;
        struct U;
        impl CanAutoIntoo<U> for T {}
        impl Froom<T> for U {
            fn froom(_: &T) -> U {
                U
            }
        }
        assert_impl_all!(T: Intoo<U>);
    }
}
