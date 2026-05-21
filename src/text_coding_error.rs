/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
mod text_decoding_error;
mod text_decoding_error_kind;
mod text_encoding_error;
mod text_encoding_error_kind;

pub use text_decoding_error::TextDecodingError;
pub use text_decoding_error_kind::TextDecodingErrorKind;
pub use text_encoding_error::TextEncodingError;
pub use text_encoding_error_kind::TextEncodingErrorKind;

use thiserror::Error;

/// Result type returned by text decoders.
pub type TextDecodingResult<T> = Result<T, TextDecodingError>;

/// Result type returned by text encoders.
pub type TextEncodingResult<T> = Result<T, TextEncodingError>;

/// Result type for callers that intentionally combine encoding and decoding errors.
pub type TextCodingResult<T> = Result<T, TextCodingError>;

/// Error wrapper for APIs that intentionally combine encoding and decoding.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum TextCodingError {
    /// A decoding failure.
    #[error(transparent)]
    Decoding(#[from] TextDecodingError),

    /// An encoding failure.
    #[error(transparent)]
    Encoding(#[from] TextEncodingError),
}
