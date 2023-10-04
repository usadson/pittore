// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::fmt::Debug;

pub trait AsDebug {
    /// The [`Debug`] trait requires a type to be [`Sized`], which makes
    /// polymorphism with e.g. [`Any`][std::any::Any] quite hard. Instead,
    /// we require the underlying type to implement this function so we can
    /// implement the [Debug] trait directly on the [AsDebug].
    fn as_debug(&self) -> &dyn Debug;
}

impl Debug for dyn AsDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_debug().fmt(f)
    }
}

impl<T> AsDebug for T
        where T: Debug {
    fn as_debug(&self) -> &dyn Debug {
        self
    }
}
