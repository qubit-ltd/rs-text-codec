/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
/// Indicates that more input units are required to decode a prefix.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NeedMore {
    /// The total number of input units required.
    pub required: usize,

    /// The number of input units currently available.
    pub available: usize,
}

impl NeedMore {
    /// Creates a need-more-input marker.
    ///
    /// # Parameters
    ///
    /// - `required`: The total number of input units required.
    /// - `available`: The number of input units currently available.
    ///
    /// # Returns
    ///
    /// Returns a new need-more-input marker.
    #[must_use]
    pub const fn new(required: usize, available: usize) -> Self {
        Self {
            required,
            available,
        }
    }
}
