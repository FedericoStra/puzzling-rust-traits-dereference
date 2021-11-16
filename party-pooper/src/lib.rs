//! This module defines a *private* struct `Qux` and implements `FooRef: CanAutoIntoo<Qux>`.

use conversions::*;

// This is not even public!
#[derive(Debug)]
struct Qux;

// This causes [`FooRef`] to satisfy `FooRef: CanAutoIntoo<??>` for some type.
impl CanAutoIntoo<Qux> for FooRef {}
