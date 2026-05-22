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
    CharsetEncodeErrorKind,
};

/// Error reported by a charset encoder.
///
/// The error always carries the target charset, error kind, and operation
/// index associated with the failure. For buffer errors this is the caller-supplied
/// output index. Errors tied to a raw code point or character value expose that
/// value through [`Self::value`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharsetEncodeError {
    /// Target charset of the operation that produced this error.
    charset: Charset,
    /// Failure category describing why encoding could not proceed.
    kind: CharsetEncodeErrorKind,
    /// Output unit index or input code point index where failure occurred.
    index: usize,
    /// Raw code point/character value that triggered the failure, if known.
    value: Option<u32>,
}

/// Result type returned by charset encoders.
///
/// # Type Parameters
///
/// - `T`: Successful value produced by an encoding operation.
pub type CharsetEncodeResult<T> = Result<T, CharsetEncodeError>;

impl CharsetEncodeError {
    /// Creates an encoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The target charset.
    /// - `kind`: The failure category.
    /// - `index`: The operation index associated with the failure.
    ///
    /// # Returns
    ///
    /// Returns an encoding error carrying the supplied context.
    #[inline]
    pub const fn new(charset: Charset, kind: CharsetEncodeErrorKind, index: usize) -> Self {
        Self {
            charset,
            kind,
            index,
            value: None,
        }
    }

    /// Creates an encoding error with an associated raw value.
    ///
    /// # Parameters
    ///
    /// - `charset`: The target charset.
    /// - `kind`: The failure category.
    /// - `index`: The operation index associated with the failure.
    /// - `value`: The raw code point or character value associated with the failure.
    ///
    /// # Returns
    ///
    /// Returns an encoding error carrying the supplied context and value.
    #[inline]
    pub const fn with_value(
        charset: Charset,
        kind: CharsetEncodeErrorKind,
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

    /// Creates an invalid-code-point encoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The target charset.
    /// - `index`: The caller-supplied operation index associated with the failure.
    /// - `value`: The invalid raw code point value.
    ///
    /// # Returns
    ///
    /// Returns an encoding error with [`CharsetEncodeErrorKind::InvalidCodePoint`].
    #[inline]
    pub const fn invalid_code_point(charset: Charset, index: usize, value: u32) -> Self {
        Self::with_value(
            charset,
            CharsetEncodeErrorKind::InvalidCodePoint { value },
            index,
            value,
        )
    }

    /// Creates an invalid-input-index encoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The target charset.
    /// - `index`: The input character index outside the provided input slice.
    ///
    /// # Returns
    ///
    /// Returns an encoding error with [`CharsetEncodeErrorKind::InvalidInputIndex`].
    #[inline]
    pub const fn invalid_input_index(charset: Charset, index: usize) -> Self {
        Self::new(
            charset,
            CharsetEncodeErrorKind::InvalidInputIndex { input_len: 0 },
            index,
        )
    }

    /// Creates an invalid-input-index encoding error with the input length.
    ///
    /// # Parameters
    ///
    /// - `charset`: The target charset.
    /// - `index`: The input index that was out of bounds.
    /// - `input_len`: The input slice length checked during encoding.
    ///
    /// # Returns
    ///
    /// Returns an encoding error with
    /// [`CharsetEncodeErrorKind::InvalidInputIndex`], including the checked input
    /// length.
    #[inline]
    pub const fn invalid_input_index_with_len(
        charset: Charset,
        index: usize,
        input_len: usize,
    ) -> Self {
        Self::new(
            charset,
            CharsetEncodeErrorKind::InvalidInputIndex { input_len },
            index,
        )
    }

    /// Creates an unmappable-character encoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The target charset.
    /// - `index`: The caller-supplied operation index associated with the failure.
    /// - `value`: The unmappable raw character value.
    ///
    /// # Returns
    ///
    /// Returns an encoding error with [`CharsetEncodeErrorKind::UnmappableCharacter`].
    #[inline]
    pub const fn unmappable_character(charset: Charset, index: usize, value: u32) -> Self {
        Self::with_value(
            charset,
            CharsetEncodeErrorKind::UnmappableCharacter { value },
            index,
            value,
        )
    }

    /// Creates a buffer-too-small encoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The target charset.
    /// - `index`: The caller-supplied output index.
    /// - `required`: Total required output units.
    /// - `available`: Available output units from `index`.
    ///
    /// # Returns
    ///
    /// Returns an encoding error with [`CharsetEncodeErrorKind::BufferTooSmall`].
    #[inline]
    pub const fn buffer_too_small(
        charset: Charset,
        index: usize,
        required: usize,
        available: usize,
    ) -> Self {
        Self::new(
            charset,
            CharsetEncodeErrorKind::BufferTooSmall {
                required,
                available,
            },
            index,
        )
    }

    /// Returns required output units for this encoding error, if reported.
    ///
    /// # Returns
    ///
    /// Returns `Some(required)` for [`CharsetEncodeErrorKind::BufferTooSmall`],
    /// otherwise `None`.
    #[inline]
    pub const fn required(self) -> Option<usize> {
        self.kind.required()
    }

    /// Returns available output units for this encoding error, if reported.
    ///
    /// # Returns
    ///
    /// Returns `Some(available)` for [`CharsetEncodeErrorKind::BufferTooSmall`],
    /// otherwise `None`.
    #[inline]
    pub const fn available(self) -> Option<usize> {
        self.kind.available()
    }

    /// Returns the target charset.
    ///
    /// # Returns
    ///
    /// Returns the stored [`Charset`].
    #[inline]
    pub const fn charset(self) -> Charset {
        self.charset
    }

    /// Returns the encoding error kind.
    ///
    /// # Returns
    ///
    /// Returns the stored [`CharsetEncodeErrorKind`].
    #[inline]
    pub const fn kind(self) -> CharsetEncodeErrorKind {
        self.kind
    }

    /// Returns the operation index associated with this error.
    ///
    /// # Returns
    ///
    /// Returns the stored index.
    #[inline]
    pub const fn index(self) -> usize {
        self.index
    }

    /// Returns the raw value associated with this error.
    ///
    /// # Returns
    ///
    /// Returns `Some(value)` when the encoder captured a raw value that caused
    /// the error, or `None` when the error is only tied to an output index.
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

impl fmt::Display for CharsetEncodeError {
    /// Formats this encoding error.
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
                "{} encoding error at index {} for value 0x{:x}: {}",
                self.charset, self.index, value, self.kind,
            )
        } else {
            write!(
                formatter,
                "{} encoding error at index {}: {}",
                self.charset, self.index, self.kind,
            )
        }
    }
}

impl Error for CharsetEncodeError {}
