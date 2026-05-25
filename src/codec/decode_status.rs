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
///
/// Values are reported for a [`crate::CharsetCodec::decode_one`] call over a
/// complete input slice and an absolute start index. `Complete` advances by a
/// positive number of units from that start index. `NeedMore` reports an
/// absolute required input length and the units currently available from the
/// same start index.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub enum DecodeStatus {
    /// A complete value was decoded from the prefix.
    Complete {
        /// The decoded Unicode scalar value.
        value: char,

        /// The number of input units consumed.
        ///
        /// This value must be greater than zero and must not exceed the units
        /// available from the decode start index.
        consumed: usize,
    },

    /// The current prefix is well-formed so far but incomplete.
    NeedMore {
        /// The absolute input length required to complete the current value.
        ///
        /// For a `decode_one(input, index)` call, this value must be greater
        /// than `input.len()` because `NeedMore` is only valid when the current
        /// slice is incomplete.
        required: usize,

        /// The number of input units currently available from the start index.
        ///
        /// For a `decode_one(input, index)` call, this is `input.len() - index`.
        available: usize,
    },
}
