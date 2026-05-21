/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
/// Non-error status reported after inspecting a decoder input prefix.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub enum DecodeStatus {
    /// A complete value was decoded from the prefix.
    Complete {
        /// The decoded Unicode scalar value.
        value: char,

        /// The number of input units consumed.
        consumed: usize,
    },

    /// The current prefix is well-formed so far but incomplete.
    NeedMore {
        /// The total number of input units required.
        required: usize,

        /// The number of input units currently available.
        available: usize,
    },
}
