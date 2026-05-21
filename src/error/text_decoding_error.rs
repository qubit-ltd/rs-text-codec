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
    TextDecodingErrorKind,
    TextEncoding,
};

/// Error reported by a text decoder.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextDecodingError {
    encoding: TextEncoding,
    kind: TextDecodingErrorKind,
    index: usize,
}

/// Result type returned by text decoders.
pub type TextDecodingResult<T> = Result<T, TextDecodingError>;

impl TextDecodingError {
    /// Creates a decoding error.
    ///
    /// # Parameters
    ///
    /// - `encoding`: The encoding being decoded.
    /// - `kind`: The failure category.
    /// - `index`: The input unit index where the failure was detected.
    ///
    /// # Returns
    ///
    /// Returns a decoding error carrying the supplied context.
    #[must_use]
    pub const fn new(encoding: TextEncoding, kind: TextDecodingErrorKind, index: usize) -> Self {
        Self {
            encoding,
            kind,
            index,
        }
    }

    /// Creates a malformed-sequence decoding error.
    ///
    /// # Parameters
    ///
    /// - `encoding`: The encoding being decoded.
    /// - `index`: The input unit index where the malformed sequence was detected.
    ///
    /// # Returns
    ///
    /// Returns a decoding error with [`TextDecodingErrorKind::MalformedSequence`].
    #[must_use]
    pub const fn malformed_sequence(encoding: TextEncoding, index: usize) -> Self {
        Self::new(encoding, TextDecodingErrorKind::MalformedSequence, index)
    }

    /// Creates an incomplete-sequence decoding error.
    ///
    /// # Parameters
    ///
    /// - `encoding`: The encoding being decoded.
    /// - `index`: The input unit index where more input was required.
    ///
    /// # Returns
    ///
    /// Returns a decoding error with [`TextDecodingErrorKind::IncompleteSequence`].
    #[must_use]
    pub const fn incomplete_sequence(encoding: TextEncoding, index: usize) -> Self {
        Self::new(encoding, TextDecodingErrorKind::IncompleteSequence, index)
    }

    /// Creates an invalid-code-point decoding error.
    ///
    /// # Parameters
    ///
    /// - `encoding`: The encoding being decoded.
    /// - `index`: The input unit index associated with the invalid code point.
    ///
    /// # Returns
    ///
    /// Returns a decoding error with [`TextDecodingErrorKind::InvalidCodePoint`].
    #[must_use]
    pub const fn invalid_code_point(encoding: TextEncoding, index: usize) -> Self {
        Self::new(encoding, TextDecodingErrorKind::InvalidCodePoint, index)
    }

    /// Returns the encoding being decoded.
    ///
    /// # Returns
    ///
    /// Returns the stored [`TextEncoding`].
    #[must_use]
    pub const fn encoding(self) -> TextEncoding {
        self.encoding
    }

    /// Returns the decoding error kind.
    ///
    /// # Returns
    ///
    /// Returns the stored [`TextDecodingErrorKind`].
    #[must_use]
    pub const fn kind(self) -> TextDecodingErrorKind {
        self.kind
    }

    /// Returns the input unit index associated with this error.
    ///
    /// # Returns
    ///
    /// Returns the index at which the error was detected.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
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
    #[must_use]
    pub const fn offset_by(self, base: usize) -> Self {
        Self {
            encoding: self.encoding,
            kind: self.kind,
            index: self.index + base,
        }
    }
}

impl fmt::Display for TextDecodingError {
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
        write!(
            formatter,
            "{} decoding error at index {}: {}",
            self.encoding, self.index, self.kind,
        )
    }
}

impl Error for TextDecodingError {}
