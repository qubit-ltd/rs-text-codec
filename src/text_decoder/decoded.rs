/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
/// A successfully decoded value and the number of input units consumed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Decoded<T> {
    /// The decoded value.
    pub value: T,

    /// The number of input units consumed.
    pub consumed: usize,
}

impl<T> Decoded<T> {
    /// Creates a decoded value record.
    ///
    /// # Parameters
    ///
    /// - `value`: The decoded value.
    /// - `consumed`: The number of input units consumed.
    ///
    /// # Returns
    ///
    /// Returns a new decoded value record.
    #[must_use]
    pub const fn new(value: T, consumed: usize) -> Self {
        Self { value, consumed }
    }
}
