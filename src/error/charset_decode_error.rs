/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use core::fmt;
use std::error::Error;

use crate::{
    Charset,
    CharsetDecodeErrorKind,
};

/// Error reported by a charset decoder.
///
/// The error always carries the charset, error kind, and input unit index at
/// which the failure was detected. Errors that decode a raw numeric value, such
/// as invalid UTF-32 units, also carry that value through [`Self::value`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharsetDecodeError {
    /// Charset being decoded when this error was detected.
    charset: Charset,
    /// Failure category describing the decoding error.
    kind: CharsetDecodeErrorKind,
    /// Input unit index at which decoding failure occurred.
    index: usize,
    /// Raw numeric value that caused the failure, if captured from input.
    value: Option<u32>,
}

/// Result type returned by charset decoders.
///
/// # Type Parameters
///
/// - `T`: Successful value produced by a decoding operation.
pub type CharsetDecodeResult<T> = Result<T, CharsetDecodeError>;

impl CharsetDecodeError {
    /// Creates a decoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The charset being decoded.
    /// - `kind`: The failure category.
    /// - `index`: The input unit index where the failure was detected.
    ///
    /// # Returns
    ///
    /// Returns a decoding error carrying the supplied context.
    #[inline]
    pub const fn new(charset: Charset, kind: CharsetDecodeErrorKind, index: usize) -> Self {
        Self {
            charset,
            kind,
            index,
            value: None,
        }
    }

    /// Creates a decoding error with an associated raw value.
    ///
    /// # Parameters
    ///
    /// - `charset`: The charset being decoded.
    /// - `kind`: The failure category.
    /// - `index`: The input unit index where the failure was detected.
    /// - `value`: The raw value associated with the failure.
    ///
    /// # Returns
    ///
    /// Returns a decoding error carrying the supplied context and value.
    #[inline]
    pub const fn with_value(
        charset: Charset,
        kind: CharsetDecodeErrorKind,
        index: usize,
        value: u32,
    ) -> Self {
        Self {
            charset,
            kind,
            index,
            value: Some(value),
        }
    }

    /// Creates a malformed-sequence decoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The charset being decoded.
    /// - `index`: The input unit index where the malformed sequence was detected.
    ///
    /// # Returns
    ///
    /// Returns a decoding error with [`CharsetDecodeErrorKind::MalformedSequence`].
    #[inline]
    pub const fn malformed_sequence(charset: Charset, index: usize) -> Self {
        Self::new(charset, CharsetDecodeErrorKind::MalformedSequence, index)
    }

    /// Creates an incomplete-sequence decoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The charset being decoded.
    /// - `index`: The input unit index where more input was required.
    ///
    /// # Returns
    ///
    /// Returns a decoding error with [`CharsetDecodeErrorKind::IncompleteSequence`].
    #[inline]
    pub const fn incomplete_sequence(charset: Charset, index: usize) -> Self {
        Self::new(charset, CharsetDecodeErrorKind::IncompleteSequence, index)
    }

    /// Creates an invalid-code-point decoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The charset being decoded.
    /// - `index`: The input unit index associated with the invalid code point.
    /// - `value`: The invalid raw code point value.
    ///
    /// # Returns
    ///
    /// Returns a decoding error with [`CharsetDecodeErrorKind::InvalidCodePoint`].
    #[inline]
    pub const fn invalid_code_point(charset: Charset, index: usize, value: u32) -> Self {
        Self::with_value(
            charset,
            CharsetDecodeErrorKind::InvalidCodePoint,
            index,
            value,
        )
    }

    /// Returns the charset being decoded.
    ///
    /// # Returns
    ///
    /// Returns the stored [`Charset`].
    #[inline]
    pub const fn charset(self) -> Charset {
        self.charset
    }

    /// Returns the decoding error kind.
    ///
    /// # Returns
    ///
    /// Returns the stored [`CharsetDecodeErrorKind`].
    #[inline]
    pub const fn kind(self) -> CharsetDecodeErrorKind {
        self.kind
    }

    /// Returns the input unit index associated with this error.
    ///
    /// # Returns
    ///
    /// Returns the index at which the error was detected.
    #[inline]
    pub const fn index(self) -> usize {
        self.index
    }

    /// Returns the raw value associated with this error.
    ///
    /// # Returns
    ///
    /// Returns `Some(value)` when the decoder captured a raw value that caused
    /// the error, or `None` when the error is only tied to an input unit index.
    #[inline]
    pub const fn value(self) -> Option<u32> {
        self.value
    }

    /// Offsets this error by a base unit index.
    ///
    /// # Parameters
    ///
    /// - `base`: The base index to add to the stored index.
    ///
    /// # Returns
    ///
    /// Returns a copy of this error with its index shifted by `base`.
    #[inline]
    pub const fn offset_by(self, base: usize) -> Self {
        Self {
            charset: self.charset,
            kind: self.kind,
            index: self.index + base,
            value: self.value,
        }
    }
}

impl fmt::Display for CharsetDecodeError {
    /// Formats this decoding error.
    ///
    /// # Parameters
    ///
    /// - `formatter`: The formatter receiving the diagnostic message.
    ///
    /// # Errors
    ///
    /// Returns any formatting error reported by `formatter`.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.value {
            write!(
                formatter,
                "{} decoding error at index {} for value 0x{:x}: {}",
                self.charset, self.index, value, self.kind,
            )
        } else {
            write!(
                formatter,
                "{} decoding error at index {}: {}",
                self.charset, self.index, self.kind,
            )
        }
    }
}

impl Error for CharsetDecodeError {}
