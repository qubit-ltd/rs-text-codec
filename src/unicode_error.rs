/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use std::error::Error;
use std::fmt;

use crate::UnicodeErrorKind;

/// Result type used by low-level Unicode cursor and encoder operations.
pub type UnicodeResult<T> = Result<T, UnicodeError>;

/// Error reported by low-level Unicode operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnicodeError {
    kind: UnicodeErrorKind,
    index: usize,
}

impl UnicodeError {
    /// Creates a new error with the specified kind and input or output index.
    #[must_use]
    pub const fn new(kind: UnicodeErrorKind, index: usize) -> Self {
        Self { kind, index }
    }

    /// Returns the error kind.
    #[must_use]
    pub const fn kind(self) -> UnicodeErrorKind {
        self.kind
    }

    /// Returns the index at which the error was detected.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
}

impl fmt::Display for UnicodeError {
    /// Formats this error for diagnostics.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} at index {}", self.kind.message(), self.index)
    }
}

impl Error for UnicodeError {}
