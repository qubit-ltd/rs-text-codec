/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
/// Classifies errors reported by low-level Unicode operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnicodeErrorKind {
    /// The output buffer does not have enough room for the encoded code point.
    BufferOverflow,

    /// The input code-unit sequence is malformed.
    MalformedUnicode,

    /// The input code-unit sequence ends before a full code point is available.
    IncompleteUnicode,
}

impl UnicodeErrorKind {
    /// Returns the Java-compatible numeric error code for this error kind.
    #[must_use]
    pub const fn code(self) -> i32 {
        match self {
            Self::BufferOverflow => -2,
            Self::MalformedUnicode => -4,
            Self::IncompleteUnicode => -5,
        }
    }

    /// Returns a stable human-readable description of this error kind.
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Self::BufferOverflow => "The buffer overflows.",
            Self::MalformedUnicode => "The Unicode code unit sequence is malformed.",
            Self::IncompleteUnicode => "The Unicode code unit sequence is incomplete.",
        }
    }
}
